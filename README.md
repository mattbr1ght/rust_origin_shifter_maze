hello,

this is a maze solver, generator and loader.

# Purpose

This is a hobby project that later evolved to support loading, and flags for one of my CS classes.

# Philosophy

I strive to write modular and general code and this project is designed with that in mind. I want people to be able to easily understand the logic, and swap out modules of my code that they want kinda like modding.

# Windows CMD not displaying colors?

to fix it you need to create the following DWORD key in registry

```
[HKEY_CURRENT_USER\Console]
"VirtualTerminalLevel"=dword:00000001
```
