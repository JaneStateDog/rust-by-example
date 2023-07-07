Primitive types can be converted to each other through [[casting]].

Rust addresses conversion between custom types (i.e., [[struct]] and [[enum]]) by the use of traits. The generic conversions will use [[From and Into#From|From]] and [[From and Into#Into|Into]] traits. However there are more specific ones for the more common cases, in particular when converting to and from Strings.