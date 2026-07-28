# DEX Format: Items and IDs

## header_item
| Name | Format | Description |
| :--- | :--- | :--- |
| magic | ubyte[8] | `DEX_FILE_MAGIC` |
| checksum | uint | adler32 checksum |
| signature | ubyte[20] | SHA-1 signature |
| file_size | uint | size of the entire file |
| header_size | uint | size of the header (0x70 or 0x78) |
| endian_tag | uint | `ENDIAN_CONSTANT` |
| map_off | uint | offset to `map_list` |
| string_ids_size | uint | count of strings |
| string_ids_off | uint | offset to string identifiers |
| type_ids_size | uint | count of types |
| type_ids_off | uint | offset to type identifiers |
| proto_ids_size | uint | count of prototypes |
| proto_ids_off | uint | offset to prototype identifiers |
| field_ids_size | uint | count of fields |
| field_ids_off | uint | offset to field identifiers |
| method_ids_size | uint | count of methods |
| method_ids_off | uint | offset to method identifiers |
| class_defs_size | uint | count of classes |
| class_defs_off | uint | offset to class definitions |

## map_list
List of all contents of a file in order.
- `size`: uint - number of entries.
- `list`: `map_item[]`.

### map_item
- `type`: ushort - item type code.
- `size`: uint - count of items.
- `offset`: uint - offset from start of file.

## string_id_item
- `string_data_off`: uint - offset to `string_data_item`.

## type_id_item
- `descriptor_idx`: uint - index into `string_ids`.

## proto_id_item
- `shorty_idx`: uint - index into `string_ids`.
- `return_type_idx`: uint - index into `type_ids`.
- `parameters_off`: uint - offset to `type_list`.

## field_id_item
- `class_idx`: ushort - index into `type_ids` for the definer.
- `type_idx`: ushort - index into `type_ids` for the type.
- `name_idx`: uint - index into `string_ids` for the name.

## method_id_item
- `class_idx`: ushort - index into `type_ids` for the definer.
- `proto_idx`: ushort - index into `proto_ids` for the prototype.
- `name_idx`: uint - index into `string_ids` for the name.
