# Issue #6: Nested class - outermost ancestor determines test/non-test

# Pattern 1: TestOuter > Helper > test_foo -> outermost=TestOuter (test) -> INCLUDED
class TestOuter:
    class Helper:
        def test_nested_in_test_outer(self):
            assert True

# Pattern 2: UserService > TestInner > test_foo -> outermost=UserService (non-test) -> EXCLUDED
class UserService:
    class TestInner:
        def test_nested_in_non_test_outer(self):
            assert True

# Pattern 3: ServiceA > ServiceB > test_connection -> outermost=ServiceA (non-test) -> EXCLUDED
class ServiceA:
    class ServiceB:
        def test_connection(self):
            assert True
