// in theory rot-13 only affects the lowercase characters in a cipher
pub fn theoretical_rot13(text: &str) -> String {
    let mut pos : u8= 0;
    let mut npos : u8 =0;
    text.to_owned().chars().map(|mut c|{
    
      if c.is_ascii_lowercase(){
        // ((c as u8) + 13) as char
        pos= c as u8 - 'a' as u8;
    npos=(pos + 13)%26;
    c=(npos + 'a' as u8) as char;
        c
      }else {
          c
      }
    }).collect()
    }
  
    #[cfg(test)]
    mod tests {
        use super::*;
  
        #[test]
        fn test_single_letter() {
            assert_eq!("n", theoretical_rot13("a"));
        }
        #[test]
    fn test_bunch_of_letters() {
        assert_eq!("nop", theoretical_rot13("abc"));
    }
  
    #[test]
    fn test_non_ascii() {
        assert_eq!("😀ab", theoretical_rot13("😀no"));
    }
  
    #[test]
    fn test_twice() {
        assert_eq!("abcd", theoretical_rot13(&theoretical_rot13("abcd")));
    }
  
    }
  