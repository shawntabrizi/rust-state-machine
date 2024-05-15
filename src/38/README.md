
<div class="content-row">
<div class="content-col">

{{#include ./template/README.md}}

</div>

<div class="content-col">

<div class="tab">
  <button class="maintab tablinks active" onclick="switchMainTab(event, 'Template')">Template</button>
  <button class="maintab tablinks" onclick="switchMainTab(event, 'Solution')">Solution</button>
  <button class="maintab tablinks" onclick="switchMainTab(event, 'Diff')">Diff</button>
</div>

<div id="Template" class="maintab tabcontent active">

<div class="tab">
<button class="subtab tablinks file-template file-modified active" onclick="switchSubTab(event, 'Cargo.toml')" data-id="Cargo.toml">Cargo.toml</button>
<button class="subtab tablinks file-template file-added" onclick="switchSubTab(event, 'macros/src/lib.rs')" data-id="macros/src/lib.rs">macros/src/lib.rs</button>
</div>
<div id="template/Cargo.toml" class="subtab tabcontent active" data-id="Cargo.toml">

```toml
{{#include ./template/Cargo.toml}}
```

</div>

<div id="template/macros/src/lib.rs" class="subtab tabcontent" data-id="macros/src/lib.rs">

```rust
{{#include ./template/macros/src/lib.rs}}
```

</div>



</div>

<div id="Solution" class="maintab tabcontent">

<div class="tab">
<button class="subtab tablinks file-solution file-modified active" onclick="switchSubTab(event, 'Cargo.toml')" data-id="Cargo.toml">Cargo.toml</button>
<button class="subtab tablinks file-solution file-added" onclick="switchSubTab(event, 'macros/Cargo.toml')" data-id="macros/Cargo.toml">macros/Cargo.toml</button>
<button class="subtab tablinks file-solution file-added" onclick="switchSubTab(event, 'macros/src/call/expand.rs')" data-id="macros/src/call/expand.rs">macros/src/call/expand.rs</button>
<button class="subtab tablinks file-solution file-added" onclick="switchSubTab(event, 'macros/src/call/mod.rs')" data-id="macros/src/call/mod.rs">macros/src/call/mod.rs</button>
<button class="subtab tablinks file-solution file-added" onclick="switchSubTab(event, 'macros/src/call/parse.rs')" data-id="macros/src/call/parse.rs">macros/src/call/parse.rs</button>
<button class="subtab tablinks file-solution file-modified" onclick="switchSubTab(event, 'macros/src/lib.rs')" data-id="macros/src/lib.rs">macros/src/lib.rs</button>
<button class="subtab tablinks file-solution file-added" onclick="switchSubTab(event, 'macros/src/runtime/expand.rs')" data-id="macros/src/runtime/expand.rs">macros/src/runtime/expand.rs</button>
<button class="subtab tablinks file-solution file-added" onclick="switchSubTab(event, 'macros/src/runtime/mod.rs')" data-id="macros/src/runtime/mod.rs">macros/src/runtime/mod.rs</button>
<button class="subtab tablinks file-solution file-added" onclick="switchSubTab(event, 'macros/src/runtime/parse.rs')" data-id="macros/src/runtime/parse.rs">macros/src/runtime/parse.rs</button>
</div>
<div id="solution/Cargo.toml" class="subtab tabcontent active" data-id="Cargo.toml">

```toml
{{#include ./solution/Cargo.toml}}
```

</div>

<div id="solution/macros/Cargo.toml" class="subtab tabcontent" data-id="macros/Cargo.toml">

```toml
{{#include ./solution/macros/Cargo.toml}}
```

</div>

<div id="solution/macros/src/call/expand.rs" class="subtab tabcontent" data-id="macros/src/call/expand.rs">

```rust
{{#include ./solution/macros/src/call/expand.rs}}
```

</div>

<div id="solution/macros/src/call/mod.rs" class="subtab tabcontent" data-id="macros/src/call/mod.rs">

```rust
{{#include ./solution/macros/src/call/mod.rs}}
```

</div>

<div id="solution/macros/src/call/parse.rs" class="subtab tabcontent" data-id="macros/src/call/parse.rs">

```rust
{{#include ./solution/macros/src/call/parse.rs}}
```

</div>

<div id="solution/macros/src/lib.rs" class="subtab tabcontent" data-id="macros/src/lib.rs">

```rust
{{#include ./solution/macros/src/lib.rs}}
```

</div>

<div id="solution/macros/src/runtime/expand.rs" class="subtab tabcontent" data-id="macros/src/runtime/expand.rs">

```rust
{{#include ./solution/macros/src/runtime/expand.rs}}
```

</div>

<div id="solution/macros/src/runtime/mod.rs" class="subtab tabcontent" data-id="macros/src/runtime/mod.rs">

```rust
{{#include ./solution/macros/src/runtime/mod.rs}}
```

</div>

<div id="solution/macros/src/runtime/parse.rs" class="subtab tabcontent" data-id="macros/src/runtime/parse.rs">

```rust
{{#include ./solution/macros/src/runtime/parse.rs}}
```

</div>



</div>

<div id="Diff" class="maintab tabcontent">


<div class="tab">
	<button class="difftab tablinks active" onclick="switchDiff(event, 'template.diff')" data-id="template.diff">template.diff</button>
	<button class="difftab tablinks" onclick="switchDiff(event, 'solution.diff')" data-id="solution.diff">solution.diff</button>
</div>
<div id="template.diff" class="difftab tabcontent active" data-id="template.diff">

```diff
{{#include ./template/template.diff}}
```

</div>
<div id="solution.diff" class="difftab tabcontent" data-id="solution.diff">

```diff
{{#include ./solution/solution.diff}}
```

</div>

</div>

</div>
</div>
