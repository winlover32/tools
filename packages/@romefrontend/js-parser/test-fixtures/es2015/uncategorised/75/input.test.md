# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test packages/@romefrontend/js-parser/index.test.ts --update-snapshots` to update.

## `es2015 > uncategorised > 75`

### `ast`

```javascript
JSRoot {
	comments: Array []
	corrupt: false
	diagnostics: Array []
	directives: Array []
	filename: "input.js"
	hasHoistedVars: false
	interpreter: undefined
	mtime: undefined
	sourceType: "module"
	syntax: Array []
	loc: Object {
		filename: "input.js"
		end: Object {
			column: 25
			index: 25
			line: 1
		}
		start: Object {
			column: 0
			index: 0
			line: 1
		}
	}
	body: Array [
		JSExportLocalDeclaration {
			exportKind: "value"
			specifiers: undefined
			loc: Object {
				filename: "input.js"
				end: Object {
					column: 25
					index: 25
					line: 1
				}
				start: Object {
					column: 0
					index: 0
					line: 1
				}
			}
			declaration: JSVariableDeclarationStatement {
				loc: Object {
					filename: "input.js"
					end: Object {
						column: 25
						index: 25
						line: 1
					}
					start: Object {
						column: 7
						index: 7
						line: 1
					}
				}
				declaration: JSVariableDeclaration {
					kind: "let"
					loc: Object {
						filename: "input.js"
						end: Object {
							column: 25
							index: 25
							line: 1
						}
						start: Object {
							column: 7
							index: 7
							line: 1
						}
					}
					declarations: Array [
						JSVariableDeclarator {
							id: JSBindingIdentifier {
								name: "document"
								loc: Object {
									filename: "input.js"
									identifierName: "document"
									end: Object {
										column: 19
										index: 19
										line: 1
									}
									start: Object {
										column: 11
										index: 11
										line: 1
									}
								}
							}
							loc: Object {
								filename: "input.js"
								end: Object {
									column: 25
									index: 25
									line: 1
								}
								start: Object {
									column: 11
									index: 11
									line: 1
								}
							}
							init: JSObjectExpression {
								properties: Array []
								loc: Object {
									filename: "input.js"
									end: Object {
										column: 25
										index: 25
										line: 1
									}
									start: Object {
										column: 22
										index: 22
										line: 1
									}
								}
							}
						}
					]
				}
			}
		}
	]
}
```

### `diagnostics`

```
✔ No known problems!

```