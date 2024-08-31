# Readonly

This example shows how to create an immutable/readonly type from any other type. It achieves this by using the `Deref` trait.
It is also an example of using rustdoc with failing tests to show the user of the lib correct and incorrect examples.

You can use this type as you want, but honestly it is not that useful since types in rust are immutable by default if you do not specify the `mut` keyword.
Also since you can just `.deref()` the readonly type you can obtain the original data type and mutate it.
