![](https://github.com/drexhage/pixel/actions/workflows/ci.yaml/badge.svg)
![](https://github.com/drexhage/pixel/actions/workflows/cd.yaml/badge.svg)
![](https://raw.githubusercontent.com/drexhage/pixel/docs-coverage/coverage/badges/flat.svg)

## Local development

If you want to develop on the rust side and the svelte side **at the same time**, then you need to make some code adjustments beforehand.

1. In `package.json` adjust the wasm packages to point to the local builds

```diff
-	"@drexhage/engine": "latest",
-	"@drexhage/common-ui": "latest",
+	"@drexhage/engine": "file:///../crates/engine/pkg",
+	"@drexhage/common-ui": "file:///../crates/common-ui/pkg",
```

2. In `vite.config.js` uncomment the wasm pack instruction

```diff
-	//wasmPack(['../crates/engine', '../crates/common-ui'])
+	wasmPack(['../crates/engine', '../crates/common-ui'])
```
