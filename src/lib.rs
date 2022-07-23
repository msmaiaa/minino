mod str_utils;

use winreg::{
    enums::{HKEY_LOCAL_MACHINE, KEY_ALL_ACCESS},
    RegKey, RegValue,
};

pub struct Path {
    pub path_reg_key: RegKey,
}

impl Path {
    pub fn new() -> Path {
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let path_reg_key = hklm
            .open_subkey_with_flags(
                "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment",
                KEY_ALL_ACCESS,
            )
            .unwrap();
        Path { path_reg_key }
    }

    pub fn get_raw_value(&self) -> Result<RegValue, std::io::Error> {
        self.path_reg_key.get_raw_value("Path")
    }

    pub fn get_value(&self) -> Result<String, std::io::Error> {
        self.path_reg_key.get_value("Path")
    }

    pub fn set_raw_value(&self, value: RegValue) -> Result<(), std::io::Error> {
        self.path_reg_key.set_raw_value("Path", &value)
    }

    pub fn set_value(&self, value: String) -> Result<(), std::io::Error> {
        self.path_reg_key.set_value("Path", &value)
    }

    pub fn get_value_as_vec(&self) -> Result<Vec<String>, std::io::Error> {
        self.get_value().map(|value| {
            value
                .split(";")
                .map(|s| s.to_string())
                .filter(|v| !v.is_empty())
                .collect()
        })
    }

    pub fn parse_vec_to_value(&self, array: &Vec<String>) -> String {
        let res = array.join(";");
        match array.last() {
            Some(last) => {
                if last.ends_with("\\") {
                    res + ";"
                } else {
                    res
                }
            }
            None => res,
        }
    }

    pub fn push_value(&self, value: String) -> Result<(), std::io::Error> {
        let mut array = self.get_value_as_vec()?;
        array.push(value);
        let values = self.parse_vec_to_value(&array);
        self.set_value(values)
    }

    pub fn remove_value(&self, substr: String) -> Result<(), std::io::Error> {
        let mut values = self.get_value_as_vec()?;
        let item_idx = str_utils::find_unique_substr_in_vec(&values, &substr);
        values.remove(item_idx);
        self.set_value(self.parse_vec_to_value(&values))
    }
}

#[cfg(test)]
mod test {
    use crate::Path;
    #[test]
    fn parse_vec_to_value() {
        let path = Path::new();
        let str_values = r#"C:\Program Files\Alacritty\;C:\Python310\Scripts\;"#.to_string();
        let mock_values = vec![
            r#"C:\Program Files\Alacritty\"#.to_string(),
            r#"C:\Python310\Scripts\"#.to_string(),
        ];
        let parsed_vec_to_value = path.parse_vec_to_value(&mock_values);
        assert_eq!(str_values, parsed_vec_to_value);
    }
}
