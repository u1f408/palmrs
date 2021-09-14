var searchIndex = JSON.parse('{\
"palmrs":{"doc":"A collection of libraries and command-line utilities for …","t":[],"n":[],"q":[],"d":[],"i":[],"f":[],"p":[]},\
"palmrs_database":{"doc":"Support for reading, and eventually writing, the Palm OS …","t":[8,3,3,3,16,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,0,12,11,11,11,10,11,11,11,0,12,0,11,11,11,11,11,11,11,11,11,11,11,17,3,12,12,12,11,11,11,11,12,12,11,11,11,11,11,11,11,12,12,12,11,11,11,12,12,12,11,11,11,11,12,11,11,12,12,8,10,10,10,10,10,0,10,10,4,8,13,3,13,3,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,10,11,11,10,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,12,12,12,12,12,12,12,3,17,11,11,11,11,11,11,11,11,11,11,11,5,11,5,11,11,11,11,11,11,5],"n":["DatabaseFormat","PalmDatabase","PdbDatabase","PrcDatabase","RecordHeader","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","clone","clone_into","eq","fmt","fmt","from","from","from","from_bytes","header","header","into","into","into","is_valid","is_valid","is_valid","ne","record","records","time","to_owned","to_string","try_from","try_from","try_from","try_into","try_into","try_into","type_id","type_id","type_id","DATABASE_HEADER_LENGTH","DatabaseHeader","app_info_id","attributes","backup_time","borrow","borrow_mut","clone","clone_into","creation_time","creator_code","creator_code_try_str","eq","fmt","fmt","from","from_bytes","into","modification_number","modification_time","name","name_trimmed","name_try_str","ne","next_record_list","record_count","sort_info_id","to_owned","to_string","try_from","try_into","type_code","type_code_try_str","type_id","unique_id_seed","version","DatabaseRecord","attributes","data_len","data_offset","from_bytes","name_str","pdb_record","struct_len","write_bytes","PdbRecordHeader","PdbRecordHeaderTrait","Record","RecordHeaderType","Resource","ResourceHeaderType","attributes","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","clone","clone_into","data_len","data_offset","eq","fmt","from","from","from","from_bytes","header_type","into","into","into","name_str","ne","next_entry_data_offset","next_entry_data_offset","next_entry_data_offset","struct_len","struct_len","struct_len","struct_len","to_owned","try_from","try_from","try_from","try_into","try_into","try_into","type_id","type_id","type_id","write_bytes","attributes","data_len","data_len","data_offset","data_offset","name","record_id","unique_id","0","PalmTimestamp","SECONDS_BETWEEN_PALM_EPOCHS","as_unix_ts","borrow","borrow_mut","clone","clone_into","default","eq","fmt","fmt","from","into","is_palm_epoch","ne","palm_ts_to_unix_ts","strftime","to_owned","to_string","try_from","try_into","type_id","unix_ts_to_palm_ts"],"q":["palmrs_database","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","palmrs_database::header","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","palmrs_database::record","","","","","","","","","palmrs_database::record::pdb_record","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","palmrs_database::record::pdb_record::PdbRecordHeader","","","","","","","","palmrs_database::time","","","","","","","","","","","","","","","","","","","","","","",""],"d":["Helper trait for database format types","A representation of a Palm OS database file","Implementation of <code>DatabaseFormat</code> for PDB databases","Implementation of <code>DatabaseFormat</code> for PRC databases","The record header type for this database format","","","","","","","","","","","","","","","","The common file header used by both the PRC and PDB …","","","","","Returns whether the database is valid as this database …","","","","Individual database record handling","","<code>PalmTimestamp</code> type &amp; conversion methods","","","","","","","","","","","","Length, in bytes, of the <code>DatabaseHeader</code>","The common file header used by both the PRC and PDB …","","","","","","","","","","Attempt to convert the database creator code to a <code>str</code>","","","","","Read the database header from the given byte slice.","","","","","Return the friendly name of the database as a byte slice, …","Attempt to convert the friendly name of the database to a …","","","","","","","","","","Attempt to convert the database type code to a <code>str</code>","","","","Helper trait for database record types","Return the record’s attributes, if known","Return the length of the record’s data, if known","Return the offset, from the start of the database file, of …","Read the record header from the given byte array","Return the record’s name, if known","Palm database record headers","The length of the record header, in bytes","Write the record header to a new <code>Vec&lt;u8&gt;</code>","Generic Palm database record header","Generic record header type helper trait","","“Record” header type","","“Resource” header type","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Type representing a Palm OS timestamp","The number of seconds between the two Palm OS timestamp …","Return the timestamp as the seconds since the UNIX epoch","","","","","","","","","","","Check if the given timestamp is using the “old Palm epoch…","","Convert an “old Palm epoch” timestamp to a UNIX epoch …","Return the timestamp as a <code>strftime</code>-formatted string","","","","","","Convert a UNIX epoch timestamp to an “old Palm epoch” …"],"i":[0,0,0,0,1,2,3,4,2,3,4,4,4,4,4,4,2,3,4,4,0,4,2,3,4,1,2,3,4,0,4,0,4,4,2,3,4,2,3,4,2,3,4,0,0,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,0,6,6,6,6,6,0,6,6,0,0,7,0,7,0,7,8,9,7,8,9,7,7,7,7,7,7,7,8,9,7,7,7,8,9,7,7,7,10,8,9,10,8,9,7,7,8,9,7,8,9,7,8,9,7,7,11,11,12,11,12,12,12,11,13,0,0,13,13,13,13,13,13,13,13,13,13,13,0,13,0,13,13,13,13,13,13,0],"f":[null,null,null,null,null,[[]],[[]],[[]],[[]],[[]],[[]],[[],["palmdatabase",3]],[[]],[[["palmdatabase",3]],["bool",15]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[]],[[]],[[]],[[],[["error",3],["result",4,["error"]]]],null,null,[[]],[[]],[[]],[[["databaseheader",3]],["bool",15]],[[["databaseheader",3]],["bool",15]],[[["databaseheader",3]],["bool",15]],[[["palmdatabase",3]],["bool",15]],null,null,null,[[]],[[],["string",3]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],null,null,null,null,null,[[]],[[]],[[],["databaseheader",3]],[[]],null,null,[[],[["str",15],["result",4,["str","utf8error"]],["utf8error",3]]],[[["databaseheader",3]],["bool",15]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[]],[[],[["error",3],["result",4,["error"]]]],[[]],null,null,null,[[]],[[],[["str",15],["result",4,["str","utf8error"]],["utf8error",3]]],[[["databaseheader",3]],["bool",15]],null,null,null,[[]],[[],["string",3]],[[],["result",4]],[[],["result",4]],null,[[],[["str",15],["result",4,["str","utf8error"]],["utf8error",3]]],[[],["typeid",3]],null,null,null,[[],[["option",4,["u32"]],["u32",15]]],[[],[["option",4,["u32"]],["u32",15]]],[[],["u32",15]],[[["usize",15],["databaseheader",3]],[["error",3],["result",4,["error"]]]],[[],[["str",15],["option",4,["str"]]]],null,[[],["usize",15]],[[],[["vec",3,["u8"]],["error",3],["result",4,["vec","error"]]]],null,null,null,null,null,null,[[],[["option",4,["u32"]],["u32",15]]],[[]],[[]],[[]],[[]],[[]],[[]],[[],["pdbrecordheader",4]],[[]],[[],[["option",4,["u32"]],["u32",15]]],[[],["u32",15]],[[["pdbrecordheader",4]],["bool",15]],[[["formatter",3]],["result",6]],[[]],[[]],[[]],[[["usize",15],["databaseheader",3]],[["error",3],["result",4,["error"]]]],[[],["pdbrecordheadertrait",8]],[[]],[[]],[[]],[[],[["str",15],["option",4,["str"]]]],[[["pdbrecordheader",4]],["bool",15]],[[],["usize",15]],[[],["usize",15]],[[],["usize",15]],[[],["usize",15]],[[],["usize",15]],[[],["usize",15]],[[],["usize",15]],[[]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],[["vec",3,["u8"]],["error",3],["result",4,["vec","error"]]]],null,null,null,null,null,null,null,null,null,null,null,[[],["i32",15]],[[]],[[]],[[],["palmtimestamp",3]],[[]],[[]],[[["palmtimestamp",3]],["bool",15]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[]],[[]],[[["u32",15]],["bool",15]],[[["palmtimestamp",3]],["bool",15]],[[["u32",15]],["i32",15]],[[["str",15]],["string",3]],[[]],[[],["string",3]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[["i32",15]],["u32",15]]],"p":[[8,"DatabaseFormat"],[3,"PrcDatabase"],[3,"PdbDatabase"],[3,"PalmDatabase"],[3,"DatabaseHeader"],[8,"DatabaseRecord"],[4,"PdbRecordHeader"],[3,"RecordHeaderType"],[3,"ResourceHeaderType"],[8,"PdbRecordHeaderTrait"],[13,"Record"],[13,"Resource"],[3,"PalmTimestamp"]]},\
"palmrs_sync":{"doc":"Support for Palm HotSync, with pluggable sync conduits","t":[13,13,13,4,11,11,11,11,0,11,11,11,11,11,11,11,11,11,11,11,11,3,3,3,11,11,11,11,11,11,11,11,11,11,11,11,12,12,12,12,11,11,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,12,12,11,12,12,12,11,11,11,11,11,11,11,11,11,11,11,11],"n":["KeepDevice","KeepLocal","Merge","SyncMode","borrow","borrow_mut","clone","clone_into","conduit","default","eq","fmt","fmt","from","from_str","into","to_owned","to_string","try_from","try_into","type_id","ConduitHandler","WithinConduit","WithinConduitConfig","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","clone","clone","clone","clone_into","clone_into","clone_into","conduit_config","conduit_name","conduit_name","config","default","default","environment","eq","eq","eq","fmt","fmt","fmt","from","from","from","from_env","from_env","into","into","into","make_argv","make_config_prefix","make_environment","ne","ne","ne","new","new","path_device","path_device","path_local","path_local","popen","sync_mode","sync_mode","sync_version","to_owned","to_owned","to_owned","try_from","try_from","try_from","try_into","try_into","try_into","type_id","type_id","type_id"],"q":["palmrs_sync","","","","","","","","","","","","","","","","","","","","","palmrs_sync::conduit","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"d":["","","","","","","","","Sync conduit handling","","","","","","","","","","","","","Conduit call handler","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"i":[1,1,1,0,1,1,1,1,0,1,1,1,1,1,1,1,1,1,1,1,1,0,0,0,2,3,4,2,3,4,2,3,4,2,3,4,4,3,4,3,2,3,2,2,3,4,2,3,4,2,3,4,2,3,2,3,4,4,4,4,2,3,4,3,4,2,4,2,4,4,2,4,3,2,3,4,2,3,4,2,3,4,2,3,4],"f":[null,null,null,null,[[]],[[]],[[],["syncmode",4]],[[]],null,[[]],[[["syncmode",4]],["bool",15]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[]],[[["str",15]],["result",4]],[[]],[[]],[[],["string",3]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],null,null,null,[[]],[[]],[[]],[[]],[[]],[[]],[[],["withinconduitconfig",3]],[[],["withinconduit",3]],[[],["conduithandler",3]],[[]],[[]],[[]],null,null,null,null,[[],["withinconduitconfig",3]],[[]],null,[[["withinconduitconfig",3]],["bool",15]],[[["withinconduit",3]],["bool",15]],[[["conduithandler",3]],["bool",15]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[]],[[]],[[]],[[],[["error",3],["result",4,["error"]]]],[[],[["error",3],["result",4,["error"]]]],[[]],[[]],[[]],[[],[["vec",3,["osstring"]],["osstring",3]]],[[],["osstring",3]],[[],["vec",3]],[[["withinconduitconfig",3]],["bool",15]],[[["withinconduit",3]],["bool",15]],[[["conduithandler",3]],["bool",15]],[[["str",15]]],[[["str",15],["syncmode",4]]],null,null,null,null,[[["redirection",4]],[["result",6,["popen"]],["popen",3]]],null,null,null,[[]],[[]],[[]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]]],"p":[[4,"SyncMode"],[3,"WithinConduitConfig"],[3,"WithinConduit"],[3,"ConduitHandler"]]}\
}');
if (window.initSearch) {window.initSearch(searchIndex)};