# DEX Format: Classes and Code

## class_def_item
| Name | Format | Description |
| :--- | :--- | :--- |
| class_idx | uint | index into `type_ids` |
| access_flags | uint | access flags |
| superclass_idx | uint | index into `type_ids` or `NO_INDEX` |
| interfaces_off | uint | offset to `type_list` |
| source_file_idx | uint | index into `string_ids` or `NO_INDEX` |
| annotations_off | uint | offset to `annotations_directory_item` |
| class_data_off | uint | offset to `class_data_item` |
| static_values_off | uint | offset to `encoded_array_item` |

## class_data_item
- `static_fields_size`: uleb128
- `instance_fields_size`: uleb128
- `direct_methods_size`: uleb128
- `virtual_methods_size`: uleb128
- `static_fields`: `encoded_field[]`
- `instance_fields`: `encoded_field[]`
- `direct_methods`: `encoded_method[]`
- `virtual_methods`: `encoded_method[]`

## code_item
| Name | Format | Description |
| :--- | :--- | :--- |
| registers_size | ushort | number of registers used |
| ins_size | ushort | number of incoming argument words |
| outs_size | ushort | number of outgoing argument words |
| tries_size | ushort | number of `try_item`s |
| debug_info_off | uint | offset to debug info |
| insns_size | uint | size of instructions in 16-bit units |
| insns | ushort[] | actual bytecode |
| tries | `try_item[]` | optional array |
| handlers | `encoded_catch_handler_list` | optional catch handlers |
