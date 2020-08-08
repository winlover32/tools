# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `core > uncategorised > 522`

### `ast`

```javascript
JSRoot {
	comments: Array []
	corrupt: false
	filename: "core/uncategorised/522/input.js"
	hasHoistedVars: false
	interpreter: undefined
	mtime: undefined
	sourceType: "script"
	syntax: Array []
	loc: Object {
		filename: "core/uncategorised/522/input.js"
		end: Object {
			column: 69
			line: 1
		}
		start: Object {
			column: 0
			line: 1
		}
	}
	directives: Array [
		JSDirective {
			value: "use strict"
			loc: Object {
				filename: "core/uncategorised/522/input.js"
				end: Object {
					column: 13
					line: 1
				}
				start: Object {
					column: 0
					line: 1
				}
			}
		}
	]
	diagnostics: Array [
		Object {
			origins: Array [Object {category: "parse/js"}]
			description: Object {
				advice: Array []
				category: "parse/js"
				message: MARKUP {parts: Array [RAW_MARKUP {value: "Legacy octal literals are not allowed in strict mode"}]}
			}
			location: Object {
				filename: "core/uncategorised/522/input.js"
				mtime: undefined
				sourceText: undefined
				end: Object {
					column: 68
					line: 1
				}
				start: Object {
					column: 68
					line: 1
				}
			}
		}
	]
	body: Array [
		JSFunctionDeclaration {
			id: JSBindingIdentifier {
				name: "foo"
				loc: Object {
					filename: "core/uncategorised/522/input.js"
					identifierName: "foo"
					end: Object {
						column: 25
						line: 1
					}
					start: Object {
						column: 22
						line: 1
					}
				}
			}
			loc: Object {
				filename: "core/uncategorised/522/input.js"
				end: Object {
					column: 42
					line: 1
				}
				start: Object {
					column: 13
					line: 1
				}
			}
			head: JSFunctionHead {
				async: false
				generator: false
				hasHoistedVars: false
				params: Array []
				rest: undefined
				returnType: undefined
				thisType: undefined
				typeParameters: undefined
				loc: Object {
					filename: "core/uncategorised/522/input.js"
					end: Object {
						column: 27
						line: 1
					}
					start: Object {
						column: 25
						line: 1
					}
				}
			}
			body: JSBlockStatement {
				body: Array []
				loc: Object {
					filename: "core/uncategorised/522/input.js"
					end: Object {
						column: 42
						line: 1
					}
					start: Object {
						column: 27
						line: 1
					}
				}
				directives: Array [
					JSDirective {
						value: "use strict"
						loc: Object {
							filename: "core/uncategorised/522/input.js"
							end: Object {
								column: 41
								line: 1
							}
							start: Object {
								column: 28
								line: 1
							}
						}
					}
				]
			}
		}
		JSFunctionDeclaration {
			id: JSBindingIdentifier {
				name: "bar"
				loc: Object {
					filename: "core/uncategorised/522/input.js"
					identifierName: "bar"
					end: Object {
						column: 54
						line: 1
					}
					start: Object {
						column: 51
						line: 1
					}
				}
			}
			loc: Object {
				filename: "core/uncategorised/522/input.js"
				end: Object {
					column: 69
					line: 1
				}
				start: Object {
					column: 42
					line: 1
				}
			}
			head: JSFunctionHead {
				async: false
				generator: false
				hasHoistedVars: true
				params: Array []
				rest: undefined
				returnType: undefined
				thisType: undefined
				typeParameters: undefined
				loc: Object {
					filename: "core/uncategorised/522/input.js"
					end: Object {
						column: 56
						line: 1
					}
					start: Object {
						column: 54
						line: 1
					}
				}
			}
			body: JSBlockStatement {
				directives: Array []
				loc: Object {
					filename: "core/uncategorised/522/input.js"
					end: Object {
						column: 69
						line: 1
					}
					start: Object {
						column: 56
						line: 1
					}
				}
				body: Array [
					JSVariableDeclarationStatement {
						loc: Object {
							filename: "core/uncategorised/522/input.js"
							end: Object {
								column: 68
								line: 1
							}
							start: Object {
								column: 57
								line: 1
							}
						}
						declaration: JSVariableDeclaration {
							kind: "var"
							loc: Object {
								filename: "core/uncategorised/522/input.js"
								end: Object {
									column: 68
									line: 1
								}
								start: Object {
									column: 57
									line: 1
								}
							}
							declarations: Array [
								JSVariableDeclarator {
									id: JSBindingIdentifier {
										name: "v"
										loc: Object {
											filename: "core/uncategorised/522/input.js"
											identifierName: "v"
											end: Object {
												column: 62
												line: 1
											}
											start: Object {
												column: 61
												line: 1
											}
										}
									}
									loc: Object {
										filename: "core/uncategorised/522/input.js"
										end: Object {
											column: 68
											line: 1
										}
										start: Object {
											column: 61
											line: 1
										}
									}
									init: JSNumericLiteral {
										value: 13
										format: "octal"
										loc: Object {
											filename: "core/uncategorised/522/input.js"
											end: Object {
												column: 68
												line: 1
											}
											start: Object {
												column: 65
												line: 1
											}
										}
									}
								}
							]
						}
					}
				]
			}
		}
	]
}
```

### `diagnostics`

```

 core/uncategorised/522/input.js:1:68 parse/js ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Legacy octal literals are not allowed in strict mode

    "use strict";function foo(){"use strict";}function bar(){var v = 015}
                                                                        ^

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

✖ Found 1 problem

```