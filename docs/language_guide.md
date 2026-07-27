# SecureLang Language Guide

Welcome to SecureLang. This guide covers the basics of the syntax.

## User Authentication Blocks

A core feature of SecureLang is the built-in authentication DSL:

```securelang
user login {
  username input
  password secure
  authenticate
}
```

This block automatically provisions a secure memory region for `password` and an interactive shell interface for `username`, routing the result to the built-in SecureLang authentication backend.

## Roadmap

- Variables: `secure let x = 5;`
- Functions: `secure fn authenticate() -> bool`
- Modules: `import "auth" as auth;`
