schema Person:
    name: str  # required
    age: int   # required
    address?: str  # optional

# Valid instance - all required attributes present
person1 = Person {
    name: "John"
    age: 30
    address: "123 Main St"
}

# Invalid instance - missing required 'age' attribute
person2 = Person {
    name: "Jane"
}

# Invalid instance - missing required 'name' attribute
person3 = Person {
    age: 25
}

# Valid instance - optional attribute omitted
person4 = Person {
    name: "Bob"
    age: 40
} 