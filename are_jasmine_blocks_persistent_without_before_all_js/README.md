# Is Transient TDD Exercise (Jasmine)

This project is a checking whether each then/it block of a Jasmine test is a transient fresh fixture.

## Getting Started

1. Install dependencies:

   ```bash
   npm install
   ```

2. Run tests:

   ```bash
   npx jasmine tests/is_transient.spec.js
   ```

3. Auto-watch tests:

   ```bash
   ./watch-test.sh
   ```

## Weird setup issue

Node version defaults to a very old version on Phil’s machine. To fix this, enter:

   ```bash
   nvm use 22.11.0
   ```

## end
