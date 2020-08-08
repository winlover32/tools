# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `esprima > es2015-generator > generator-parameter-binding-property`

### `ast`

```javascript
JSRoot {
	comments: Array []
	corrupt: false
	directives: Array []
	filename: "esprima/es2015-generator/generator-parameter-binding-property/input.js"
	hasHoistedVars: false
	interpreter: undefined
	mtime: undefined
	sourceType: "script"
	syntax: Array []
	loc: Object {
		filename: "esprima/es2015-generator/generator-parameter-binding-property/input.js"
		end: Object {
			column: 0
			line: 4
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
				message: MARKUP {parts: Array [RAW_MARKUP {value: "Expected an identifier"}]}
			}
			location: Object {
				filename: "esprima/es2015-generator/generator-parameter-binding-property/input.js"
				mtime: undefined
				sourceText: undefined
				end: Object {
					column: 12
					line: 2
				}
				start: Object {
					column: 12
					line: 2
				}
			}
		}
	]
	body: Array [
		JSExpressionStatement {
			loc: Object {
				filename: "esprima/es2015-generator/generator-parameter-binding-property/input.js"
				end: Object {
					column: 2
					line: 3
				}
				start: Object {
					column: 0
					line: 1
				}
			}
			expression: JSFunctionExpression {
				id: undefined
				loc: Object {
					filename: "esprima/es2015-generator/generator-parameter-binding-property/input.js"
					end: Object {
						column: 1
						line: 3
					}
					start: Object {
						column: 1
						line: 1
					}
				}
				head: JSFunctionHead {
					async: false
					generator: true
					hasHoistedVars: false
					params: Array []
					rest: undefined
					returnType: undefined
					thisType: undefined
					typeParameters: undefined
					loc: Object {
						filename: "esprima/es2015-generator/generator-parameter-binding-property/input.js"
						end: Object {
							column: 12
							line: 1
						}
						start: Object {
							column: 10
							line: 1
						}
					}
				}
				body: JSBlockStatement {
					directives: Array []
					loc: Object {
						filename: "esprima/es2015-generator/generator-parameter-binding-property/input.js"
						end: Object {
							column: 1
							line: 3
						}
						start: Object {
							column: 13
							line: 1
						}
					}
					body: Array [
						JSFunctionDeclaration {
							id: JSBindingIdentifier {
								name: ""
								loc: Object {
									filename: "esprima/es2015-generator/generator-parameter-binding-property/input.js"
									identifierName: ""
									end: Object {
										column: 13
										line: 2
									}
									start: Object {
										column: 12
										line: 2
									}
								}
							}
							loc: Object {
								filename: "esprima/es2015-generator/generator-parameter-binding-property/input.js"
								end: Object {
									column: 33
									line: 2
								}
								start: Object {
									column: 4
									line: 2
								}
							}
							body: JSBlockStatement {
								body: Array []
								directives: Array []
								loc: Object {
									filename: "esprima/es2015-generator/generator-parameter-binding-property/input.js"
									end: Object {
										column: 33
										line: 2
									}
									start: Object {
										column: 31
										line: 2
									}
								}
							}
							head: JSFunctionHead {
								async: false
								generator: false
								hasHoistedVars: false
								rest: undefined
								returnType: undefined
								thisType: undefined
								typeParameters: undefined
								loc: Object {
									filename: "esprima/es2015-generator/generator-parameter-binding-property/input.js"
									end: Object {
										column: 30
										line: 2
									}
									start: Object {
										column: 13
										line: 2
									}
								}
								params: Array [
									JSBindingObjectPattern {
										rest: undefined
										loc: Object {
											filename: "esprima/es2015-generator/generator-parameter-binding-property/input.js"
											end: Object {
												column: 29
												line: 2
											}
											start: Object {
												column: 13
												line: 2
											}
										}
										meta: JSPatternMeta {
											optional: undefined
											typeAnnotation: undefined
											loc: Object {
												filename: "esprima/es2015-generator/generator-parameter-binding-property/input.js"
												end: Object {
													column: 29
													line: 2
												}
												start: Object {
													column: 13
													line: 2
												}
											}
										}
										properties: Array [
											JSBindingObjectPatternProperty {
												key: JSStaticPropertyKey {
													value: JSIdentifier {
														name: "x"
														loc: Object {
															filename: "esprima/es2015-generator/generator-parameter-binding-property/input.js"
															identifierName: "x"
															end: Object {
																column: 15
																line: 2
															}
															start: Object {
																column: 14
																line: 2
															}
														}
													}
													loc: Object {
														filename: "esprima/es2015-generator/generator-parameter-binding-property/input.js"
														end: Object {
															column: 15
															line: 2
														}
														start: Object {
															column: 14
															line: 2
														}
													}
												}
												value: JSBindingAssignmentPattern {
													loc: Object {
														filename: "esprima/es2015-generator/generator-parameter-binding-property/input.js"
														end: Object {
															column: 26
															line: 2
														}
														start: Object {
															column: 17
															line: 2
														}
													}
													left: JSBindingIdentifier {
														name: "y"
														loc: Object {
															filename: "esprima/es2015-generator/generator-parameter-binding-property/input.js"
															identifierName: "y"
															end: Object {
																column: 18
																line: 2
															}
															start: Object {
																column: 17
																line: 2
															}
														}
													}
													right: JSReferenceIdentifier {
														name: "yield"
														loc: Object {
															filename: "esprima/es2015-generator/generator-parameter-binding-property/input.js"
															identifierName: "yield"
															end: Object {
																column: 26
																line: 2
															}
															start: Object {
																column: 21
																line: 2
															}
														}
													}
												}
												loc: Object {
													filename: "esprima/es2015-generator/generator-parameter-binding-property/input.js"
													end: Object {
														column: 26
														line: 2
													}
													start: Object {
														column: 14
														line: 2
													}
												}
											}
										]
									}
								]
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

 esprima/es2015-generator/generator-parameter-binding-property/input.js:2:12 parse/js ━━━━━━━━━━━━━━

  ✖ Expected an identifier

    1 │ (function*() {
  > 2 │     function({x: y = yield 3}) {}
      │             ^
    3 │ })

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

✖ Found 1 problem

```