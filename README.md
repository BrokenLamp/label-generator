# label-generator

A command line utility to generate n^k labels based on label components.

## Getting Started

For this example, we'll assume we are a screw company that needs labels for each type and length of screw.

For instance:

- Phillips 2"
- Phillips 3"
- Phillips 4"
- Flat-head 2"
- Flat-head 3"
- Flat-head 4"

Create a new folder

```bash
mkdir labels
cd labels
```

Create a `manifest.toml` file

```toml
root = "root.svg"
sku = "SC-{head}-{length}"
```

Create a `root.svg` file

```svg
<?xml version="1.0" encoding="utf-8"?>
<svg version="1.1" id="Layer_1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" x="0px" y="0px"
	 width="2048px" height="2048px" viewBox="0 0 2048 2048" style="enable-background:new 0 0 2048 2048;" xml:space="preserve">

<!-- component:brand -->
<!-- component:head -->
<!-- component:length -->

</svg>
```

Create a new folder for our expanding components

```bash
mkdir head
mkdir length
```

Add files to these folders to be injected into `root.svg`.

```
└── head
    ├── P-phillips.svg
    └── F-flat_head.svg
└── length
    ├── 2-inch.svg
    ├── 3-inch.svg
    └── 4-inch.svg
├── branding.svg
├── manifest.toml
└── root.svg
```

Now we run the label generator

```
$ label-generator

SVG Label Generator

🔧 Config:
  >> Root: root.svg
  >> SKU: SC-{head}-{length}
📦 Components:
  >> branding 📝
  >> head 📂
  >> length 📂
💾 Generated Files:
  >> SC-P-2
  >> SC-P-3
  >> SC-P-4
  >> SC-F-2
  >> SC-F-3
  >> SC-F-4
✅ Done
```

This will create an `out` directory with our new label files

```
└── out
    ├── SC-P-2
    ├── SC-P-3
    ├── SC-P-4
    ├── SC-F-2
    ├── SC-F-3
    └── SC-F-4
```
