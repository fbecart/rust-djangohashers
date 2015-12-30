#[macro_use]
extern crate djangohashers;


#[cfg(test)]
mod tests {

    use djangohashers::*;

    static PASSWORD: &'static str = "ExjGmyUT73bFoT";
    static SALT: &'static str = "KQ8zeK6wKRuR";

    #[test]
    fn test_pbkdf2_sha256() {
        let encoded = make_password_with_settings(PASSWORD, SALT, Algorithm::PBKDF2);
        let h = "pbkdf2_sha256$24000$KQ8zeK6wKRuR$cmhbSt1XVKuO4FGd9+AX8qSBD4Z0395nZatXTJpEtTY=";
        assert!(encoded == h.to_string());
        assert!(check_password(PASSWORD, &encoded).unwrap());
    }

    #[test]
    fn test_pbkdf2_sha1() {
        let encoded = make_password_with_settings(PASSWORD, SALT, Algorithm::PBKDF2SHA1);
        let h = "pbkdf2_sha1$24000$KQ8zeK6wKRuR$tSJh4xdxfMJotlxfkCGjTFpGYZU=";
        assert!(encoded == h.to_string());
        assert!(check_password(PASSWORD, &encoded).unwrap());
    }

    #[test]
    fn test_sha1() {
        let encoded = make_password_with_settings(PASSWORD, SALT, Algorithm::SHA1);
        let h = "sha1$KQ8zeK6wKRuR$f83371bca01fa6089456e673ccfb17f42d810b00";
        assert!(encoded == h.to_string());
        assert!(check_password(PASSWORD, &encoded).unwrap());
    }

    #[test]
    fn test_md5() {
        let encoded = make_password_with_settings(PASSWORD, SALT, Algorithm::MD5);
        let h = "md5$KQ8zeK6wKRuR$0137e4d74cb2d9ed9cb1a5f391f6175e";
        assert!(encoded == h.to_string());
        assert!(check_password(PASSWORD, &encoded).unwrap());
    }

    #[test]
    fn test_unsalted_md5() {
        let encoded = make_password_with_settings(PASSWORD, "", Algorithm::UnsaltedMD5);
        let h = "7cf6409a82cd4c8b96a9ecf6ad679119";
        assert!(encoded == h.to_string());
        assert!(check_password(PASSWORD, &encoded).unwrap());
    }

    #[test]
    fn test_unsalted_sha1() {
        let encoded = make_password_with_settings(PASSWORD, "", Algorithm::UnsaltedSHA1);
        let h = "sha1$$22e6217f026c7a395f0840c1ffbdb163072419e7";
        assert!(encoded == h.to_string());
        assert!(check_password(PASSWORD, &encoded).unwrap());
    }

    #[test]
    fn test_bcrypt_sha256() {
        let encoded = make_password_with_settings(PASSWORD, "", Algorithm::BCryptSHA256);
        assert!(check_password(PASSWORD, &encoded).unwrap());
        let h = "bcrypt_sha256$$2b$12$LZSJchsWG/DrBy1erNs4eeYo6tZNlLFQmONdxN9HPesa1EyXVcTXK";
        assert!(check_password(PASSWORD, h).unwrap());
    }

    #[test]
    fn test_bcrypt() {
        let encoded = make_password_with_settings(PASSWORD, "", Algorithm::BCrypt);
        assert!(check_password(PASSWORD, &encoded).unwrap());
        let h = "bcrypt$$2b$12$LZSJchsWG/DrBy1erNs4ee31eJ7DaWiuwhDOC7aqIyqGGggfu6Y/.";
        assert!(check_password(PASSWORD, h).unwrap());
    }

}