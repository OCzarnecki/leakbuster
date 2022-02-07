# Deploying to AUR
It should be possible to install leakbuster via AUR, and use it as an outsider.
- [x] document configuration file
- [x] package it so I can install it locally
- [ ] figure out how to deploy that package to AUR
- [x] have default locations for db and config, overwritable for dev

# Delay startup
- [x] make delayed startup command
- [x] implement arg parsing
- [x] refactor to use controller, instead of custom widget
- [x] align text to center
- [ ] watch for screen activity

# Better Hooks
- [ ] Check conditions in StartupHook and TimeHook
- [x] Implement TimeHook execution
- [ ] Add Shutdown hooks

# Refactorings
- [x] Db should be a struct
- [ ] Parse conditions once, when loading config
- [ ] Make loop speed configurable
- [ ] After condition language stabilized a bit
  - [ ] Test expression parsing
  - [ ] Test expression evaluation

# Future Features
- [ ] Track window activity for executed apps, and make it possible to distinguish between runtime and activity in condition language
- [ ] plot / export nice metrics about app usage
- [ ] pass arguments to command, after --
