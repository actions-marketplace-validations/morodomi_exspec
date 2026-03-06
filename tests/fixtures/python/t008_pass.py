from pydantic import BaseModel


class User(BaseModel):
    name: str
    age: int


def test_user_validation():
    user = User(name="Alice", age=30)
    assert user.name == "Alice"
