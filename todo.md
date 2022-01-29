# Deploying to AUR
It should be possible to install leakbuster via AUR, and use it as an outsider.
- [ ] document configuration file
- [x] package it so I can install it locally
- [ ] figure out how to deploy that package to AUR
- [x] have default locations for db and config, overwritable for dev

# Delay startup
- [x] make delayed startup command
- [x] implement arg parsing
- [x] refactor to use controller, instead of custom widget
- [x] align text to center
- [ ] watch for screen activity

# Refactorings
- [ ] Db should be a struct
- [ ] Parse conditions once, when loading config
- [ ] Test expression parsing
- [ ] Test expression evaluation
- [ ] Replace PathBufs with Paths in Config, Db, bin/, delay, and run
