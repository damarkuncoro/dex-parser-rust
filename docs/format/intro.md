# DEX Format: Introduction & Layout

## Guide to types

| Name | Description |
| :--- | :--- |
| byte | 8-bit signed int |
| ubyte | 8-bit unsigned int |
| short | 16-bit signed int, little-endian |
| ushort | 16-bit unsigned int, little-endian |
| int | 32-bit signed int, little-endian |
| uint | 32-bit unsigned int, little-endian |
| long | 64-bit signed int, little-endian |
| ulong | 64-bit unsigned int, little-endian |
| sleb128 | signed LEB128, variable-length |
| uleb128 | unsigned LEB128, variable-length |
| uleb128p1 | unsigned LEB128 plus `1`, variable-length |

### LEB128
LEB128 ("**L**ittle-**E**ndian **B**ase **128**") is a variable-length encoding for arbitrary signed or unsigned integer quantities. In a `.dex` file, LEB128 is only ever used to encode 32-bit quantities.

## File layout

| Name | Format | Description |
| :--- | :--- | :--- |
| header | [header_item](./items.md#header_item) | the header |
| string_ids | [string_id_item[]](./items.md#string_id_item) | string identifiers list |
| type_ids | [type_id_item[]](./items.md#type_id_item) | type identifiers list |
| proto_ids | [proto_id_item[]](./items.md#proto_id_item) | method prototype identifiers list |
| field_ids | [field_id_item[]](./items.md#field_id_item) | field identifiers list |
| method_ids | [method_id_item[]](./items.md#method_id_item) | method identifiers list |
| class_defs | [class_def_item[]](./classes.md#class_def_item) | class definitions list |
| call_site_ids | [call_site_id_item[]](./values.md#call_site_id_item) | call site identifiers list |
| method_handles | [method_handle_item[]](./values.md#method_handle_item) | method handles list |
| data | ubyte[] | data area |
| link_data | ubyte[] | data used in statically linked files |

### Container format (v41+)
Version 41 introduces a new container format allowing several logical DEX files to be combined into a single physical file. All offsets are relative to the physical file.
