var searchIndex = JSON.parse('{\
"palmrs":{"doc":"","t":[],"n":[],"q":[],"d":[],"i":[],"f":[],"p":[]},\
"palmrs_database":{"doc":"Support for reading, and eventually writing, the Palm OS …","t":[8,3,16,11,11,11,11,12,11,11,11,0,11,11,0,12,11,10,11,11,0,0,11,11,11,11,11,3,3,11,11,11,11,11,11,11,12,11,12,11,11,11,11,11,11,11,11,11,12,11,11,11,11,12,11,11,11,11,11,11,11,11,11,17,3,12,12,12,11,11,11,11,12,12,11,11,11,11,11,11,11,12,12,12,11,11,11,12,12,12,11,11,11,11,12,11,11,12,12,8,3,11,11,10,10,11,10,11,11,11,11,10,11,10,11,11,11,12,3,17,11,11,11,11,11,11,11,11,11,11,11,5,11,5,11,11,11,11,11,11,5],"n":["DatabaseFormat","PalmDatabase","RecordHeader","borrow","borrow_mut","clone","clone_into","data","eq","fmt","fmt","format_prc","from","from_bytes","header","header","into","is_valid","iter_records","ne","record","time","to_owned","to_string","try_from","try_into","type_id","PrcDatabase","PrcRecordHeader","borrow","borrow","borrow_mut","borrow_mut","clone","clone_into","data_len","data_len","data_offset","data_offset","eq","fmt","fmt","from","from","from_bytes","into","into","is_valid","name","name_str","name_trimmed","name_try_str","ne","record_id","struct_len","to_owned","to_string","try_from","try_from","try_into","try_into","type_id","type_id","DATABASE_HEADER_LENGTH","DatabaseHeader","app_info_id","attributes","backup_time","borrow","borrow_mut","clone","clone_into","creation_time","creator_code","creator_code_try_str","eq","fmt","fmt","from","from_bytes","into","modification_number","modification_time","name","name_trimmed","name_try_str","ne","next_record_list","record_count","sort_info_id","to_owned","to_string","try_from","try_into","type_code","type_code_try_str","type_id","unique_id_seed","version","DatabaseRecord","RecordIter","borrow","borrow_mut","data_len","data_offset","from","from_bytes","from_bytes","from_database","into","into_iter","name_str","next","struct_len","try_from","try_into","type_id","0","PalmTimestamp","SECONDS_BETWEEN_PALM_EPOCHS","as_unix_ts","borrow","borrow_mut","clone","clone_into","default","eq","fmt","fmt","from","into","is_palm_epoch","ne","palm_ts_to_unix_ts","strftime","to_owned","to_string","try_from","try_into","type_id","unix_ts_to_palm_ts"],"q":["palmrs_database","","","","","","","","","","","","","","","","","","","","","","","","","","","palmrs_database::format_prc","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","palmrs_database::header","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","palmrs_database::record","","","","","","","","","","","","","","","","","","palmrs_database::time","","","","","","","","","","","","","","","","","","","","","","",""],"d":["Helper trait for database format types","A representation of a Palm OS database file","The record header type for this database format","","","","","","","","","The PRC (Palm Resource Code) database format","","","The common file header used by both the PRC and PDB …","","","Returns whether the database is valid as this database …","","","Individual database record handling","<code>PalmTimestamp</code> type &amp; conversion methods","","","","","","Implementation of <code>DatabaseFormat</code> for PRC databases","A PRC record header","","","","","","","","","","","","","","","","","","","","","","Return the name of the record as a trimmed byte slice","Attempt to convert the record name to a <code>str</code>","","","","","","","","","","","","Length, in bytes, of the <code>DatabaseHeader</code>","The common file header used by both the PRC and PDB …","","","","","","","","","","Attempt to convert the database creator code to a <code>str</code>","","","","","Read the database header from the given byte slice.","","","","","Return the friendly name of the database as a byte slice, …","Attempt to convert the friendly name of the database to a …","","","","","","","","","","Attempt to convert the database type code to a <code>str</code>","","","","Helper trait for database record types","Iterator over the records in a database","","","Return the length of the record’s data, if known.","Return the offset, from the start of the database file, of …","","Read the record header from the given byte array.","","","","","Return the record’s name, if known","","The length of the record header, in bytes","","","","","Type representing a Palm OS timestamp","The number of seconds between the two Palm OS timestamp …","Return the timestamp as the seconds since the UNIX epoch","","","","","","","","","","","Check if the given timestamp is using the “old Palm epoch…","","Convert an “old Palm epoch” timestamp to a UNIX epoch …","Return the timestamp as a <code>strftime</code>-formatted string","","","","","","Convert a UNIX epoch timestamp to an “old Palm epoch” …"],"i":[0,0,1,2,2,2,2,2,2,2,2,0,2,2,0,2,2,1,2,2,0,0,2,2,2,2,2,0,0,3,4,3,4,4,4,4,4,4,4,4,4,4,3,4,4,3,4,3,4,4,4,4,4,4,4,4,4,3,4,3,4,3,4,0,0,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,0,0,6,6,7,7,6,7,6,6,6,6,7,6,7,6,6,6,8,0,0,8,8,8,8,8,8,8,8,8,8,8,0,8,0,8,8,8,8,8,8,0],"f":[null,null,null,[[]],[[]],[[],["palmdatabase",3]],[[]],null,[[["palmdatabase",3]],["bool",15]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],null,[[]],[[],[["error",3],["result",4,["error"]]]],null,null,[[]],[[["databaseheader",3]],["bool",15]],[[],["recorditer",3]],[[["palmdatabase",3]],["bool",15]],null,null,[[]],[[],["string",3]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],null,null,[[]],[[]],[[]],[[]],[[],["prcrecordheader",3]],[[]],[[],[["option",4,["u32"]],["u32",15]]],null,[[],["u32",15]],null,[[["prcrecordheader",3]],["bool",15]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[]],[[]],[[["usize",15]],[["error",3],["result",4,["error"]]]],[[]],[[]],[[["databaseheader",3]],["bool",15]],null,[[],[["str",15],["option",4,["str"]]]],[[]],[[],[["str",15],["result",4,["str","utf8error"]],["utf8error",3]]],[[["prcrecordheader",3]],["bool",15]],null,[[],["usize",15]],[[]],[[],["string",3]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],null,null,null,null,null,[[]],[[]],[[],["databaseheader",3]],[[]],null,null,[[],[["str",15],["result",4,["str","utf8error"]],["utf8error",3]]],[[["databaseheader",3]],["bool",15]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[]],[[],[["error",3],["result",4,["error"]]]],[[]],null,null,null,[[]],[[],[["str",15],["result",4,["str","utf8error"]],["utf8error",3]]],[[["databaseheader",3]],["bool",15]],null,null,null,[[]],[[],["string",3]],[[],["result",4]],[[],["result",4]],null,[[],[["str",15],["result",4,["str","utf8error"]],["utf8error",3]]],[[],["typeid",3]],null,null,null,null,[[]],[[]],[[],[["option",4,["u32"]],["u32",15]]],[[],["u32",15]],[[]],[[["usize",15]],[["error",3],["result",4,["error"]]]],[[],[["error",3],["result",4,["error"]]]],[[["palmdatabase",3]]],[[]],[[]],[[],[["str",15],["option",4,["str"]]]],[[],["option",4]],[[],["usize",15]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],null,null,null,[[],["i32",15]],[[]],[[]],[[],["palmtimestamp",3]],[[]],[[]],[[["palmtimestamp",3]],["bool",15]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[]],[[]],[[["u32",15]],["bool",15]],[[["palmtimestamp",3]],["bool",15]],[[["u32",15]],["i32",15]],[[["str",15]],["string",3]],[[]],[[],["string",3]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[["i32",15]],["u32",15]]],"p":[[8,"DatabaseFormat"],[3,"PalmDatabase"],[3,"PrcDatabase"],[3,"PrcRecordHeader"],[3,"DatabaseHeader"],[3,"RecordIter"],[8,"DatabaseRecord"],[3,"PalmTimestamp"]]}\
}');
if (window.initSearch) {window.initSearch(searchIndex)};