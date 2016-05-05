Steps:

0. Assemble a list of lexer-parser resources and db building resources.

1. Query language, 1st iter

Build a minimal query language that can filter a list of tuples by schema.

Data:

```
people = {
	schema: (:name, :address, :title),
	data: [
		("Veljko", "Igora 4", "Framework author"),
		("Mihael", "Mikija 2", "Direktor"),
		("Nikica", "Franje 7", "Programer"),
	]
}
```

Query:

`select from people where address = "Franje 7"`
