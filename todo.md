# Conditional startup prevention
It should be possible to start an application only if it's been used for less than 15 minutes on that day. Otherwise, a notification is displayed.
- [x] adapt query language
- [x] implement startup hooks
- [x] implement conditional startup hooks (done atm by chaining hook with
      `leakbuster eval` command)
- [x] implement CLI

# Deploying to AUR
It should be possible to install leakbuster via AUR, and use it as an outsider.
- [ ] document configuration file
- [ ] package it so I can install it locally
- [ ] figure out how to deploy that package to AUR
- [ ] have default locations for db and config, overwritable for dev

# Refactorings
- [ ] Db should be a struct
- [ ] Parse conditions once, when loading config
- [ ] Test expression parsing
- [ ] Test expression evaluation
