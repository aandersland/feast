---
name: run-tests
description: Automatically invoked when running tests, analyzing test results, or verifying code changes. Ensures tests are run appropriately and results are interpreted correctly.
---

# Run Tests Skill

This skill activates when you need to run tests, analyze test output, or verify code changes.

## When This Skill Activates

Automatically engage when:
- Implementing or modifying code
- Fixing bugs
- Completing a feature
- Before committing changes
- Investigating test failures
- Validating refactoring
- After dependency updates

## Test Running Principles

### Run Appropriate Test Level
- **Unit tests** - During development, after each change
- **Integration tests** - After unit tests pass, before committing
- **E2E tests** - Before merging, for critical flows
- **Full suite** - Before release, in CI/CD

### Interpret Results Correctly
- Understand failure root causes
- Distinguish test failures from code failures
- Identify flaky tests
- Recognize false positives/negatives

### Fix Failures Systematically
- One failure at a time
- Fix tests that reveal real bugs first
- Update tests that are outdated
- Remove tests that are no longer valuable

## Test Commands Reference

### Common Test Commands

```bash
# Run all tests
npm test
# or
yarn test
# or
pytest
# or
cargo test

# Run specific test file
npm test path/to/test.spec.js
pytest tests/test_user.py
cargo test user_tests

# Run tests matching pattern
npm test -- --grep "user"
pytest -k "user"
cargo test user

# Run with coverage
npm test -- --coverage
pytest --cov=src tests/
cargo test --coverage

# Run in watch mode
npm test -- --watch
pytest --watch
cargo watch -x test

# Run only failed tests
npm test -- --only-failures
pytest --lf
```

## Test Running Workflow

### 1. Identify What to Test

**Changed a function?** Run unit tests for that function
```bash
npm test user.service.spec.js
```

**Changed an API endpoint?** Run integration tests
```bash
npm test -- --grep "POST /api/users"
```

**Changed UI component?** Run component tests
```bash
npm test UserProfile.test.jsx
```

**Fixed a bug?** Run tests that cover that code path

**Refactored?** Run full test suite

### 2. Run Tests

Execute appropriate test command from project root

### 3. Analyze Results

#### Success ✓
```
PASS  tests/user.service.spec.js
  UserService
    ✓ creates user with valid data (15ms)
    ✓ throws error for invalid email (8ms)
    ✓ throws error for duplicate email (12ms)

Test Suites: 1 passed, 1 total
Tests:       3 passed, 3 total
Time:        2.145s
```

**Action:** Proceed with confidence

#### Failure ✗
```
FAIL  tests/user.service.spec.js
  UserService
    ✓ creates user with valid data (15ms)
    ✗ throws error for invalid email (23ms)
    ✓ throws error for duplicate email (12ms)

  ● UserService › throws error for invalid email

    expect(received).toThrow(expected)

    Expected: "Invalid email"
    Received: function did not throw

      18 |   it('throws error for invalid email', () => {
      19 |     const invalidUser = { email: 'invalid', name: 'Test' }
    > 20 |     expect(() => createUser(invalidUser)).toThrow('Invalid email')
         |                                            ^
      21 |   })
```

**Action:** Investigate and fix

### 4. Investigate Failures

#### Read Error Message
- What was expected?
- What was received?
- What line failed?

#### Check Code
- Is the test correct?
- Is the code correct?
- Is there a race condition?
- Is test data stale?

#### Reproduce Locally
- Run test in isolation
- Run with debugger
- Add console.logs
- Check test assumptions

### 5. Fix Issues

#### Code Bug
Fix the code, then re-run tests

#### Test Bug
Fix the test, then re-run

#### Flaky Test
- Identify timing issues
- Add proper async handling
- Mock time-dependent functions
- Increase timeouts cautiously

#### Outdated Test
Update test to match new requirements

### 6. Verify Fix

Re-run tests to confirm they pass

### 7. Run Broader Tests

After local tests pass, run:
- Related tests
- Full test suite
- Integration tests
- E2E tests (if applicable)

## Test Output Interpretation

### All Pass
```
Test Suites: 5 passed, 5 total
Tests:       47 passed, 47 total
```
**Meaning:** All tests executed successfully
**Action:** Proceed with commit/deployment

### Some Fail
```
Test Suites: 3 failed, 2 passed, 5 total
Tests:       12 failed, 35 passed, 47 total
```
**Meaning:** Code changes broke existing functionality
**Action:** Investigate and fix failures before proceeding

### Test Errors
```
Error: Cannot find module 'some-module'
```
**Meaning:** Test setup or dependency issue
**Action:** Check dependencies, run `npm install`

### Timeout
```
Timeout - Async callback was not invoked within timeout
```
**Meaning:** Async operation didn't complete
**Action:** Check async handling, increase timeout if legitimate

### Coverage Report
```
File           | % Stmts | % Branch | % Funcs | % Lines |
---------------|---------|----------|---------|---------|
user.service.js|   85.71 |    75.00 |   83.33 |   85.71 |
All files      |   85.71 |    75.00 |   83.33 |   85.71 |
```
**Meaning:** Percentage of code covered by tests
**Action:** Review uncovered lines, add tests for critical paths

## Test Types and When to Run

### Unit Tests
**What:** Individual functions, methods, classes
**When:** During development, after each code change
**Speed:** Very fast (< 1 second)
**Command:** `npm test path/to/unit.test.js`

### Integration Tests
**What:** Component interactions, API endpoints, database operations
**When:** After unit tests pass, before committing
**Speed:** Fast to moderate (seconds)
**Command:** `npm test:integration` or similar

### E2E Tests
**What:** Complete user flows, critical business processes
**When:** Before merging, before deployment
**Speed:** Slow (minutes)
**Command:** `npm test:e2e` or similar

### Smoke Tests
**What:** Basic functionality checks
**When:** After deployment, after major changes
**Speed:** Fast
**Command:** `npm test:smoke` or similar

## Coverage Analysis

### Running Coverage
```bash
npm test -- --coverage
```

### Understanding Coverage
- **Statements:** % of executable statements run
- **Branches:** % of if/else branches taken
- **Functions:** % of functions called
- **Lines:** % of lines executed

### Coverage Targets
- **Critical paths:** 90%+
- **Business logic:** 80%+
- **Utilities:** 80%+
- **Overall:** 70-80%

### Coverage Gaps
```
-----------------|---------|----------|---------|---------|
File             | % Stmts | % Branch | % Funcs | % Lines |
-----------------|---------|----------|---------|---------|
user.service.js  |   85.71 |    50.00 |   83.33 |   85.71 | 45-47
```
**Uncovered lines: 45-47**

**Action:** Review those lines, add tests if critical

## Debugging Test Failures

### Run Single Test
```bash
npm test -- --testNamePattern="creates user"
```

### Run with Verbose Output
```bash
npm test -- --verbose
```

### Add Debug Statements
```javascript
it('creates user', () => {
  console.log('Input:', userData)
  const result = createUser(userData)
  console.log('Result:', result)
  expect(result.id).toBeDefined()
})
```

### Use Debugger
```javascript
it('creates user', () => {
  debugger; // Add breakpoint
  const result = createUser(userData)
  expect(result.id).toBeDefined()
})
```

### Check Test Isolation
```bash
# Run test alone
npm test user.spec.js

# Run with other tests
npm test
```

If it passes alone but fails with others → test interdependence issue

## Common Test Issues

### Flaky Tests
**Symptom:** Sometimes pass, sometimes fail
**Causes:**
- Timing/race conditions
- Shared state between tests
- External dependencies
- Random data

**Solutions:**
- Proper async/await usage
- Reset state between tests
- Mock external dependencies
- Use deterministic test data

### Slow Tests
**Symptom:** Tests take too long
**Causes:**
- Too many E2E tests
- Not using mocks
- Database operations
- Network requests

**Solutions:**
- Mock external dependencies
- Use test doubles
- Optimize test setup
- Run critical tests only during development

### False Positives
**Symptom:** Tests pass but code is broken
**Causes:**
- Testing implementation not behavior
- Insufficient assertions
- Mock returning incorrect data

**Solutions:**
- Test observable behavior
- Add specific assertions
- Verify mock data matches reality

### False Negatives
**Symptom:** Tests fail but code is correct
**Causes:**
- Outdated tests
- Overly strict assertions
- Test environment issues

**Solutions:**
- Update tests to match requirements
- Relax overly specific assertions
- Fix test environment

## Test Maintenance

### After Fixing Bug
- Add test that would have caught the bug
- Verify test fails without fix
- Verify test passes with fix

### After Refactoring
- Run full test suite
- All tests should still pass
- If tests need updates, update them
- Coverage should remain same or improve

### After Changing Requirements
- Update tests to match new requirements
- Remove tests for removed features
- Add tests for new functionality

### Regular Maintenance
- Remove flaky tests or fix them
- Update outdated tests
- Remove redundant tests
- Improve slow tests

## Best Practices

### Run Tests Before Committing
```bash
npm test && git commit -m "message"
```

### Use Watch Mode During Development
```bash
npm test -- --watch
```

### Fix Tests Immediately
Don't let broken tests linger

### Trust Your Tests
If tests pass, code is likely correct
If tests fail, investigate immediately

### Keep Tests Fast
Fast tests = run frequently = catch bugs early

## Integration with Development Workflow

### Before Starting Work
```bash
git pull
npm install
npm test  # Ensure starting point is good
```

### During Development
```bash
npm test -- --watch  # Continuous feedback
```

### Before Committing
```bash
npm test            # All tests pass?
npm test -- --coverage  # Coverage acceptable?
git add .
git commit -m "message"
```

### Before Pushing
```bash
npm test            # Final verification
git push
```

## CI/CD Integration

Tests should run automatically in CI/CD:
- On every push
- On every pull request
- Before deployment

**Check CI/CD status** before merging or deploying

## References

- Test framework documentation: Check project's testing framework
- Coverage tools: Jest, NYC, Coverage.py, Tarpaulin
- Testing best practices: See `ai_docs/knowledge/testing/`

## Constraints

- Don't skip tests to save time
- Don't ignore test failures
- Don't commit broken tests
- Don't over-mock - test real behavior when possible
- Keep tests maintainable and readable
