# Leakbuster

> Leakbuster is a Linux tool for monitoring the time for which applications are running, and for triggering configurable events once the usage time meets some condition.

That sounds a bit abstract. You might ask: Why would I want this? Here's some use cases:

* Prevent yourself from starting discord during work hours
* Show a delay screen for 10 seconds before opening your feed reader, to make sure that's really what you want to do
* Disable your screensaver when starting VLC, and re-enable it when you're done
* Get a notification every 20 minutes while coding, to remind you to change your posture
* ...

Leakbuster is being developed to help you take control of your digital environment. But it is highly hackable, and intended to be usable in a broad range of situations. The name is an homage to the browser addon [https://www.proginosko.com/leechblock/](LeachBlock) which gave me the inspiration for leakbuster (I'm not affiliated with them in any way, however).

# Example use cases

Create a config file at `~/.config/leakbuster.config`, put one of the snippets in and try it out!

## Showing a delay screen before starting an application

This is useful if you want to break a habit of opening up some distraction (email, messenger, etc.) unthinkingly, without making it impossible to use the app if you really want to. The delay gives you time to reflect whether you really want to do something.

Use the following config:

```yaml
apps:
  - messenger
    cmd: /usr/bin/distracting-app
    startup_hooks:
      - cmd: leakbuster
        args: [delay, 10]
```

If you use

```bash
leakbuster run
```

instead of starting the app directly, it will display a screen with a countdown before starting the app. You'll be able to abort the countdown and not start the app. This screen is a subcommand of leakbuster. The configuration file isn't doing anything special, it is just running `leakbuster delay 10`.

On its own, this won't help you break the habit, since you're most likely running the distracting app via your desktop manager, without typing a command. To make it useful, see the next section:

## Making your desktop manager run your app via leakbuster

To figure out which applications to display in menus, most desktop managers and tools like dmenu or rofi use `.desktop` files, which specify the app name, icon, and the command that's run. The directories that will be searched for `.desktop` files are usually `~/.local/share/applications`, `/usr/loca/share/applications`, and `/usr/share/applications`, in that order. We can therefore make the desktop manager use leakbuster by doing the following:

1. Locate the `.desktop` file of the application, for example `/usr/share/applications/discord.desktop`.
2. Create a copy in `~/.local/share/applications/`. This way the modification we're about to do will persist across updates.
3. Find the line that defines the executable, for example 
  
  ```
  Exec=/usr/bin/discord
  ```
  
  and replace it with the right leakbuster command:
  
  ```
  Exec=/usr/bin/leakbuster run discord_id
  ```
  
  where in this example `discord_id` is the id you specified for your app configuration in the leakbuster configuration file.

If the app takes parameters, you can use the following syntax:

```
leakbuster run APP_ID -- ARG1 ARG2 ...
```

which will work fine even if the argument list is empty.

# Installation
I haven't figured this out yet to be honest, and I probably won't bother unless there's interest in this project, so I suppose:

1. Make sure you've got a rust development environment (i.p. `cargo`) installed.
2. Run `cargo build --release` from within the root of the directory.
3. Copy `target/release/leakbuster` to `/usr/bin/leakbuster`.

Alternatively, use `cargo install --path .` which will place the binary in `~/.carggo/bin/`.

If you'd like proper installation support for your plattform, raise an issue and I'll add it.
