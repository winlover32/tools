---
title: Lint Rule jsx/noDuplicateProps
layout: layouts/rule.liquid
description: MISSING DOCUMENTATION
eleventyNavigation:
	key: lint-rules/jsx/noDuplicateProps
	parent: lint-rules
	title: jsx/noDuplicateProps
---

# jsx/noDuplicateProps

MISSING DOCUMENTATION

<!-- GENERATED:START(hash:73dd5dffbd5d92c74383359ac77491cbfb9380c5,id:main) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples
### Invalid
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">Hello</span> <span class="token attr-name">foo</span><span class="token operator">=</span><span class="token string">&apos;bar&apos;</span> <span class="token attr-name">foo</span><span class="token operator">=</span><span class="token string">&apos;baz&apos;</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:7</span> <strong>lint/jsx/noDuplicateProps</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid duplicate component props. Check the </span><span style="color: Tomato;"><strong>foo</strong></span><span style="color: Tomato;"> prop.</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Defined already here</span>

    &lt;<span class="token attr-name">Hello</span> <span class="token attr-name">foo</span><span class="token operator">=</span><span class="token string">&apos;bar&apos;</span> <span class="token attr-name">foo</span><span class="token operator">=</span><span class="token string">&apos;baz&apos;</span> <span class="token operator">/</span>&gt;
                     <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">div</span> <span class="token attr-name">style</span><span class="token operator">=</span><span class="token string">&apos;{}&apos;</span> <span class="token attr-name">style</span><span class="token operator">=</span><span class="token string">&apos;{}&apos;</span> <span class="token attr-name">id</span><span class="token operator">=</span><span class="token string">&apos;foo&apos;</span> <span class="token attr-name">id</span><span class="token operator">=</span><span class="token string">&apos;bar&apos;</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:5</span> <strong>lint/jsx/noDuplicateProps</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid duplicate component props. Check the </span><span style="color: Tomato;"><strong>style</strong></span><span style="color: Tomato;"> prop.</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Defined already here</span>

    &lt;<span class="token attr-name">div</span> <span class="token attr-name">style</span><span class="token operator">=</span><span class="token string">&apos;{}&apos;</span> <span class="token attr-name">style</span><span class="token operator">=</span><span class="token string">&apos;{}&apos;</span> <span class="token attr-name">id</span><span class="token operator">=</span><span class="token string">&apos;foo&apos;</span> <span class="token attr-name">id</span><span class="token operator">=</span><span class="token string">&apos;bar&apos;</span> <span class="token operator">/</span>&gt;
                    <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

 <span style="text-decoration-style: dashed; text-decoration-line: underline;">file.tsx:1:27</span> <strong>lint/jsx/noDuplicateProps</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">Avoid duplicate component props. Check the </span><span style="color: Tomato;"><strong>id</strong></span><span style="color: Tomato;"> prop.</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Defined already here</span>

</code></pre>{% endraw %}
### Valid
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">Hello</span> <span class="token attr-name">foo</span><span class="token operator">=</span><span class="token string">&apos;bar&apos;</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">&lt;<span class="token attr-name">div</span>  <span class="token attr-name">style</span><span class="token operator">=</span><span class="token string">&apos;{}&apos;</span> <span class="token operator">/</span>&gt;</code></pre>{% endraw %}
<!-- GENERATED:END(id:main) -->