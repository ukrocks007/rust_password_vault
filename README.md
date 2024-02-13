# Password Vault in Rust

```
Usage 
Password_helper <identifier>             - to get password for identifier
Password_helper <identifier> <password>  - to set/update password for identifier
Password_helper -d <identifier>          - to delete identifier
```

In case the password has special characters use single quotes.

Example:

```bash
Password_helper RDS '12wqsfg$cdgdfg$uu67yu'
```