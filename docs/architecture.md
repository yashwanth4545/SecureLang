# SecureLang Architecture

SecureLang is an enterprise-grade programming language designed for absolute security from the ground up.

## The Compiler Engine (Rust)

- **Lexer & Parser**: Custom LL(1) parser for the SecureLang DSL.
- **AST Generation**: Statically typed abstract syntax trees.
- **Semantic Analyzer**: Checks for type safety, ownership rules, and memory violations.
- **Code Generator**: Currently translates into high-performance Rust execution blocks.

## The Web Backend (Node.js & Express)

- **Auth Layer**: Uses Argon2 for password hashing, JWT for session tokens.
- **Database**: PostgreSQL handled via Prisma ORM for type safety and SQLi prevention.
- **Security Middlewares**: Helmet (HSTS, CSP), CORS, Rate Limiting.

## The Web Frontend (React & Vite)

- **UI System**: TailwindCSS + Framer Motion.
- **Design Language**: Glassmorphism with dark theme out of the box.
- **Routing**: React Router DOM.
