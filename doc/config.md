# Configuration file specification

The entire behaviour of leakbuster (which programs can be run, the associated time and startup hooks etc.) is controlled with a configuration file. By default, leakbuster will try to load configuration from ~/.config/leakbuster.config, but an alternative file can be supplied via command line argument.

The configuration file is in YAML, and expects a root object at the top.

# Root

| Field name | Type    | Optional | Description |
| -----------|---------|----------|------------ |
| apps       | \[App\] | no       | List of apps that leakbuster can start. |

# App

| Field name     | Type             | Optional | Description |
| ---------------|------------------|----------|------------ |
| id             | text             | no       | Id by which the app is known to leakbuster. Alphanumerical IDs without spaces are recommended. |
| cmd            | text             |  no      | Command that leakbuster will run to start the app. |
| args           | \[text\]         | yes      | Arguments to pass when starting the app. |
| startup_hoo ks | \[StartupHook\]  | yes      | List of StartupHook, to be run before the app. The StartupHooks are run in order. If one of them returns a non-zero exit code, leakbuster will terminate instead of running the next one or the app. |
| time_hooks     | \[TimeHook\]     | yes      | List of TimeHooks, to be run after the app is started. See TimeHook configuration for details. |
| shutdown_hooks | \[ShutdownHook\] | yes      | List of ShutdownHooks, to be run after the application terminates (including SIGINT). ShutdownHooks will not run, if the regular startup of the application was prevented by a StartupHook. |

# StartupHook

| Field name    | Type            | Optional | Description |
| --------------|-----------------|----------|------------ |
| condition     | text            | yes      | Expression in the condition language, of type Condition. This startup hook will only be run if the expression evaluates to true. |
| cmd           | text            | no       | Command to execute in order to run this StartupHook. |
| args          | \[text\]        | yes      | Command line arguments. |

# ShutdownHook

| Field name    | Type            | Optional | Description |
| --------------|-----------------|----------|------------ |
| condition     | text            | yes      | Expression in the condition language, of type Condition. This shutdown hook will only be run if the expression evaluates to true. |
| cmd           | text            | no       | Command to execute in order to run this StartupHook. |
| args          | \[text\]        | yes      | Command line arguments. |

# TimeHook

| Field name     | Type            | Optional | Description |
| ---------------|-----------------|----------|------------ |
| cmd            | text            | no       | Command to execute in order to run this TimeHook. |
| args           | \[text\]        | yes      | Command line arguments to TimeHook command. |
| condition_cmd  | text            | yes      | Command to execute to check whether this TimeHook should be run. The TimeHook will be run only if this returns with exit code 0. |
| comdition_args | \[text\]        | yes      | Command line arguments to condition command. Only allowed if `condition_cmd` is set. |
| condition      | text            | yes      | Expression in the condition language, of type Condition. The TimeHook will only be run, if this expression evaluates to true. |
| interval       | text            | yes      | Expression in the condition language, of type Duration. The time in between consecutive runs of this time hook. If 0, this time hook will run only once. Default: 10s. |
| initial_delay  | text            | yes      | Expression in the condition language, of type Duration. The time that must elapse before the start off the application until the StartupHook is run for the first time. |
