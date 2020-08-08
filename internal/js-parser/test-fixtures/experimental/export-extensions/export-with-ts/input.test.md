# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `experimental > export-extensions > export-with-ts`

### `ast`

```javascript
JSRoot {
	comments: Array []
	corrupt: false
	directives: Array []
	filename: "experimental/export-extensions/export-with-ts/input.js"
	hasHoistedVars: false
	interpreter: undefined
	mtime: undefined
	sourceType: "module"
	syntax: Array []
	loc: Object {
		filename: "experimental/export-extensions/export-with-ts/input.js"
		end: Object {
			column: 0
			line: 8
		}
		start: Object {
			column: 0
			line: 1
		}
	}
	diagnostics: Array [
		Object {
			origins: Array [Object {category: "parse/js"}]
			description: Object {
				advice: Array []
				category: "parse/js"
				message: MARKUP {parts: Array [RAW_MARKUP {value: "Expected `from` for an export node"}]}
			}
			location: Object {
				filename: "experimental/export-extensions/export-with-ts/input.js"
				mtime: undefined
				sourceText: undefined
				end: Object {
					column: 15
					line: 1
				}
				start: Object {
					column: 16
					line: 1
				}
			}
		}
	]
	body: Array [
		JSExportExternalDeclaration {
			exportKind: undefined
			namedSpecifiers: Array []
			namespaceSpecifier: undefined
			loc: Object {
				filename: "experimental/export-extensions/export-with-ts/input.js"
				end: Object {
					column: 15
					line: 1
				}
				start: Object {
					column: 0
					line: 1
				}
			}
			source: JSStringLiteral {
				value: ""
				loc: Object {
					filename: "experimental/export-extensions/export-with-ts/input.js"
					end: Object {
						column: 15
						line: 1
					}
					start: Object {
						column: 16
						line: 1
					}
				}
			}
			defaultSpecifier: JSExportDefaultSpecifier {
				loc: Object {
					filename: "experimental/export-extensions/export-with-ts/input.js"
					end: Object {
						column: 15
						line: 1
					}
					start: Object {
						column: 7
						line: 1
					}
				}
				exported: JSIdentifier {
					name: "abstract"
					loc: Object {
						filename: "experimental/export-extensions/export-with-ts/input.js"
						identifierName: "abstract"
						end: Object {
							column: 15
							line: 1
						}
						start: Object {
							column: 7
							line: 1
						}
					}
				}
			}
		}
		JSClassDeclaration {
			id: JSBindingIdentifier {
				name: "A"
				loc: Object {
					filename: "experimental/export-extensions/export-with-ts/input.js"
					identifierName: "A"
					end: Object {
						column: 23
						line: 1
					}
					start: Object {
						column: 22
						line: 1
					}
				}
			}
			loc: Object {
				filename: "experimental/export-extensions/export-with-ts/input.js"
				end: Object {
					column: 26
					line: 1
				}
				start: Object {
					column: 16
					line: 1
				}
			}
			meta: JSClassHead {
				body: Array []
				implements: undefined
				superClass: undefined
				superTypeParameters: undefined
				typeParameters: undefined
				loc: Object {
					filename: "experimental/export-extensions/export-with-ts/input.js"
					end: Object {
						column: 26
						line: 1
					}
					start: Object {
						column: 16
						line: 1
					}
				}
			}
		}
		JSExportExternalDeclaration {
			exportKind: undefined
			namedSpecifiers: Array []
			namespaceSpecifier: undefined
			loc: Object {
				filename: "experimental/export-extensions/export-with-ts/input.js"
				end: Object {
					column: 14
					line: 2
				}
				start: Object {
					column: 0
					line: 2
				}
			}
			source: JSStringLiteral {
				value: ""
				loc: Object {
					filename: "experimental/export-extensions/export-with-ts/input.js"
					end: Object {
						column: 14
						line: 2
					}
					start: Object {
						column: 15
						line: 2
					}
				}
			}
			defaultSpecifier: JSExportDefaultSpecifier {
				loc: Object {
					filename: "experimental/export-extensions/export-with-ts/input.js"
					end: Object {
						column: 14
						line: 2
					}
					start: Object {
						column: 7
						line: 2
					}
				}
				exported: JSIdentifier {
					name: "declare"
					loc: Object {
						filename: "experimental/export-extensions/export-with-ts/input.js"
						identifierName: "declare"
						end: Object {
							column: 14
							line: 2
						}
						start: Object {
							column: 7
							line: 2
						}
					}
				}
			}
		}
		TSInterfaceDeclaration {
			id: JSBindingIdentifier {
				name: "B"
				loc: Object {
					filename: "experimental/export-extensions/export-with-ts/input.js"
					identifierName: "B"
					end: Object {
						column: 26
						line: 2
					}
					start: Object {
						column: 25
						line: 2
					}
				}
			}
			extends: undefined
			typeParameters: undefined
			loc: Object {
				filename: "experimental/export-extensions/export-with-ts/input.js"
				end: Object {
					column: 29
					line: 2
				}
				start: Object {
					column: 15
					line: 2
				}
			}
			body: TSInterfaceBody {
				body: Array []
				loc: Object {
					filename: "experimental/export-extensions/export-with-ts/input.js"
					end: Object {
						column: 29
						line: 2
					}
					start: Object {
						column: 27
						line: 2
					}
				}
			}
		}
		JSExportExternalDeclaration {
			exportKind: undefined
			namedSpecifiers: Array []
			namespaceSpecifier: undefined
			loc: Object {
				filename: "experimental/export-extensions/export-with-ts/input.js"
				end: Object {
					column: 11
					line: 3
				}
				start: Object {
					column: 0
					line: 3
				}
			}
			source: JSStringLiteral {
				value: ""
				loc: Object {
					filename: "experimental/export-extensions/export-with-ts/input.js"
					end: Object {
						column: 11
						line: 3
					}
					start: Object {
						column: 12
						line: 3
					}
				}
			}
			defaultSpecifier: JSExportDefaultSpecifier {
				loc: Object {
					filename: "experimental/export-extensions/export-with-ts/input.js"
					end: Object {
						column: 11
						line: 3
					}
					start: Object {
						column: 7
						line: 3
					}
				}
				exported: JSIdentifier {
					name: "enum"
					loc: Object {
						filename: "experimental/export-extensions/export-with-ts/input.js"
						identifierName: "enum"
						end: Object {
							column: 11
							line: 3
						}
						start: Object {
							column: 7
							line: 3
						}
					}
				}
			}
		}
		JSExpressionStatement {
			loc: Object {
				filename: "experimental/export-extensions/export-with-ts/input.js"
				end: Object {
					column: 13
					line: 3
				}
				start: Object {
					column: 12
					line: 3
				}
			}
			expression: JSReferenceIdentifier {
				name: "C"
				loc: Object {
					filename: "experimental/export-extensions/export-with-ts/input.js"
					identifierName: "C"
					end: Object {
						column: 13
						line: 3
					}
					start: Object {
						column: 12
						line: 3
					}
				}
			}
		}
		JSBlockStatement {
			body: Array []
			directives: Array []
			loc: Object {
				filename: "experimental/export-extensions/export-with-ts/input.js"
				end: Object {
					column: 16
					line: 3
				}
				start: Object {
					column: 14
					line: 3
				}
			}
		}
		JSExportLocalDeclaration {
			exportKind: "type"
			specifiers: undefined
			loc: Object {
				filename: "experimental/export-extensions/export-with-ts/input.js"
				end: Object {
					column: 21
					line: 4
				}
				start: Object {
					column: 0
					line: 4
				}
			}
			declaration: TSInterfaceDeclaration {
				id: JSBindingIdentifier {
					name: "D"
					loc: Object {
						filename: "experimental/export-extensions/export-with-ts/input.js"
						identifierName: "D"
						end: Object {
							column: 18
							line: 4
						}
						start: Object {
							column: 17
							line: 4
						}
					}
				}
				extends: undefined
				typeParameters: undefined
				loc: Object {
					filename: "experimental/export-extensions/export-with-ts/input.js"
					end: Object {
						column: 21
						line: 4
					}
					start: Object {
						column: 7
						line: 4
					}
				}
				body: TSInterfaceBody {
					body: Array []
					loc: Object {
						filename: "experimental/export-extensions/export-with-ts/input.js"
						end: Object {
							column: 21
							line: 4
						}
						start: Object {
							column: 19
							line: 4
						}
					}
				}
			}
		}
		JSExportExternalDeclaration {
			exportKind: undefined
			namedSpecifiers: Array []
			namespaceSpecifier: undefined
			loc: Object {
				filename: "experimental/export-extensions/export-with-ts/input.js"
				end: Object {
					column: 13
					line: 5
				}
				start: Object {
					column: 0
					line: 5
				}
			}
			source: JSStringLiteral {
				value: ""
				loc: Object {
					filename: "experimental/export-extensions/export-with-ts/input.js"
					end: Object {
						column: 13
						line: 5
					}
					start: Object {
						column: 14
						line: 5
					}
				}
			}
			defaultSpecifier: JSExportDefaultSpecifier {
				loc: Object {
					filename: "experimental/export-extensions/export-with-ts/input.js"
					end: Object {
						column: 13
						line: 5
					}
					start: Object {
						column: 7
						line: 5
					}
				}
				exported: JSIdentifier {
					name: "module"
					loc: Object {
						filename: "experimental/export-extensions/export-with-ts/input.js"
						identifierName: "module"
						end: Object {
							column: 13
							line: 5
						}
						start: Object {
							column: 7
							line: 5
						}
					}
				}
			}
		}
		JSExpressionStatement {
			loc: Object {
				filename: "experimental/export-extensions/export-with-ts/input.js"
				end: Object {
					column: 15
					line: 5
				}
				start: Object {
					column: 14
					line: 5
				}
			}
			expression: JSReferenceIdentifier {
				name: "E"
				loc: Object {
					filename: "experimental/export-extensions/export-with-ts/input.js"
					identifierName: "E"
					end: Object {
						column: 15
						line: 5
					}
					start: Object {
						column: 14
						line: 5
					}
				}
			}
		}
		JSBlockStatement {
			body: Array []
			directives: Array []
			loc: Object {
				filename: "experimental/export-extensions/export-with-ts/input.js"
				end: Object {
					column: 18
					line: 5
				}
				start: Object {
					column: 16
					line: 5
				}
			}
		}
		JSExportExternalDeclaration {
			exportKind: undefined
			namedSpecifiers: Array []
			namespaceSpecifier: undefined
			loc: Object {
				filename: "experimental/export-extensions/export-with-ts/input.js"
				end: Object {
					column: 16
					line: 6
				}
				start: Object {
					column: 0
					line: 6
				}
			}
			source: JSStringLiteral {
				value: ""
				loc: Object {
					filename: "experimental/export-extensions/export-with-ts/input.js"
					end: Object {
						column: 16
						line: 6
					}
					start: Object {
						column: 17
						line: 6
					}
				}
			}
			defaultSpecifier: JSExportDefaultSpecifier {
				loc: Object {
					filename: "experimental/export-extensions/export-with-ts/input.js"
					end: Object {
						column: 16
						line: 6
					}
					start: Object {
						column: 7
						line: 6
					}
				}
				exported: JSIdentifier {
					name: "namespace"
					loc: Object {
						filename: "experimental/export-extensions/export-with-ts/input.js"
						identifierName: "namespace"
						end: Object {
							column: 16
							line: 6
						}
						start: Object {
							column: 7
							line: 6
						}
					}
				}
			}
		}
		JSExpressionStatement {
			loc: Object {
				filename: "experimental/export-extensions/export-with-ts/input.js"
				end: Object {
					column: 18
					line: 6
				}
				start: Object {
					column: 17
					line: 6
				}
			}
			expression: JSReferenceIdentifier {
				name: "F"
				loc: Object {
					filename: "experimental/export-extensions/export-with-ts/input.js"
					identifierName: "F"
					end: Object {
						column: 18
						line: 6
					}
					start: Object {
						column: 17
						line: 6
					}
				}
			}
		}
		JSBlockStatement {
			body: Array []
			directives: Array []
			loc: Object {
				filename: "experimental/export-extensions/export-with-ts/input.js"
				end: Object {
					column: 21
					line: 6
				}
				start: Object {
					column: 19
					line: 6
				}
			}
		}
		JSExportLocalDeclaration {
			exportKind: "type"
			specifiers: undefined
			loc: Object {
				filename: "experimental/export-extensions/export-with-ts/input.js"
				end: Object {
					column: 27
					line: 7
				}
				start: Object {
					column: 0
					line: 7
				}
			}
			declaration: TSTypeAlias {
				id: JSBindingIdentifier {
					name: "G"
					loc: Object {
						filename: "experimental/export-extensions/export-with-ts/input.js"
						identifierName: "G"
						end: Object {
							column: 13
							line: 7
						}
						start: Object {
							column: 12
							line: 7
						}
					}
				}
				typeParameters: undefined
				loc: Object {
					filename: "experimental/export-extensions/export-with-ts/input.js"
					end: Object {
						column: 27
						line: 7
					}
					start: Object {
						column: 7
						line: 7
					}
				}
				right: TSTypeQuery {
					loc: Object {
						filename: "experimental/export-extensions/export-with-ts/input.js"
						end: Object {
							column: 26
							line: 7
						}
						start: Object {
							column: 16
							line: 7
						}
					}
					exprName: JSReferenceIdentifier {
						name: "foo"
						loc: Object {
							filename: "experimental/export-extensions/export-with-ts/input.js"
							identifierName: "foo"
							end: Object {
								column: 26
								line: 7
							}
							start: Object {
								column: 23
								line: 7
							}
						}
					}
				}
			}
		}
	]
}
```

### `diagnostics`

```

 experimental/export-extensions/export-with-ts/input.js:1:16 parse/js ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Expected `from` for an export node

  > 1 │ export abstract class A {}
      │                 ^
    2 │ export declare interface B {}
    3 │ export enum C {}

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

✖ Found 1 problem

```