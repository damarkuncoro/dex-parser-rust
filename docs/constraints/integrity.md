# General .dex Integrity Constraints

These constraints are concerned with the larger structure of a `.dex` file.

| ID | Description |
| :--- | :--- |
| **G1** | The `magic` number must be `dex\n035\0` for version 35, or similar for later versions. |
| **G2** | The checksum must be an Adler-32 checksum of the whole file contents except `magic` and `checksum` fields. |
| **G3** | The signature must be a SHA-1 hash of the whole file contents except `magic`, `checksum`, and `signature`. |
| **G4** | `file_size` must match the actual file size. For v41+, it must point to the next header in the container or the end of the file. |
| **G5** | `header_size` must be `0x70` (v40 or earlier) or `0x78` (v41 or later). |
| **G6** | `endian_tag` must be `ENDIAN_CONSTANT` or `REVERSE_ENDIAN_CONSTANT`. |
| **G7** | For `link`, `string_ids`, `type_ids`, `proto_ids`, `field_ids`, `method_ids`, `class_defs`, and `data` sections: `offset` and `size` must be both zero or both non-zero. Offsets must be four-byte-aligned. |
| **G8** | All offset fields in the header except `map_off` must be four-byte-aligned. |
| **G9** | `map_off` must be either zero or point into the data section. |
| **G10** | Sections must not overlap each other or the header. |
| **G11** | If a map exists, each entry must have a valid type. Each type may appear at most once. |
| **G12** | Map entries must have non-zero offset and size, pointing into the corresponding section. |
| **G13** | Map entries must be non-overlapping and in low-to-high order. |
| **G14** | Alignment: `string_id_item`, `type_id_item`, `proto_id_item`, `field_id_item`, `method_id_item`, `class_def_item`, `type_list`, `code_item`, `annotations_directory_item` must be four-byte-aligned. |
| **G15** | `string_id_item` must contain valid reference into data section. `string_data_item` must contain valid MUTF-8 and correct `utf16_size`. |
| **G16** | `type_id_item` descriptor index must be a valid reference to a string representing a valid type descriptor. |
| **G17** | `proto_id_item` must have valid references for shorty, return type, and parameters. |
| **G18** | `field_id_item` must have valid indices for class (non-array), type, and name. |
| **G19** | `method_id_item` must have valid indices for class (non-array), prototype, and name. |
| **G20** | `field_id_item` class index must be a valid index into `type_ids` and be a non-array reference type. |
