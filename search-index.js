var N = null;var searchIndex = {};
searchIndex["hrx"]={"doc":"","items":[[3,"HrxArchive","hrx","A Human-Readable Archive, consisting of an optional comment and some entries, all separated by the boundary.",N,N],[12,"comment","","Some optional metadata.",0,N],[12,"entries","","Some optional archive entries with their paths.",0,N],[3,"HrxEntry","","A single entry in the archive, consisting of an optional comment and some data.",N,N],[12,"comment","","Some optional metadata.",1,N],[12,"data","","The specific entry data.",1,N],[3,"HrxPath","","Verified-valid path to an entry in the archive.",N,N],[4,"ErroneousBodyPath","","A path to a `body` which contains an invalid sequence",N,N],[13,"RootComment","","The root archive comment",2,N],[13,"EntryComment","","A comment to the entry with the specified path",2,N],[13,"EntryData","","The data of the entry with the specified path",2,N],[4,"HrxError","","Generic error type, encompassing more precise errors.",N,N],[13,"NoBoundary","","No valid HRX boundary found",3,N],[13,"Parse","","An error occured during parsing",3,N],[13,"BodyContainsBoundary","","A body was made to contain the archive boundary. Deserialising the archive would not work",3,N],[4,"HrxEntryData","","Some variant of an entry's contained data.",N,N],[13,"File","","File with some optional contents.",4,N],[12,"body","hrx::HrxEntryData","",4,N],[13,"Directory","hrx","Bodyless directory.",4,N],[0,"parse","","Individual parsing primitives.",N,N],[3,"ParseError","hrx::parse","HRX parsing error ",N,N],[12,"line","","1-based line # of error ",5,N],[12,"column","","1-based column # of error ",5,N],[12,"offset","","Byte offset of error ",5,N],[12,"expected","","Expected but unmatched rules ",5,N],[5,"discover_first_boundary_length","","Search the specified for the length of the first `boundary`.",N,[[["s"]],["option",["nonzerousize"]]]],[5,"directory","","`boundary \" \"+ path \"/\" newline+`",N,[[["str"],["nonzerousize"]],["parseresult",["hrxpath"]]]],[5,"archive","","`entry* comment?`",N,[[["str"],["nonzerousize"]],["parseresult",["hrxarchive"]]]],[5,"comment","","`boundary newline body`",N,[[["str"],["nonzerousize"]],["parseresult",["str"]]]],[5,"entry","","`comment? (file | directory)`",N,[[["str"],["nonzerousize"]],["parseresult"]]],[5,"body","","`contents newline`",N,[[["str"],["nonzerousize"]],["parseresult",["str"]]]],[5,"file","","`boundary \" \"+ path newline body?`",N,[[["str"],["nonzerousize"]],["parseresult"]]],[5,"path","","`path-component (\"/\" path-component)*`",N,[[["str"],["nonzerousize"]],["parseresult",["hrxpath"]]]],[6,"ParseResult","","Convenience result type ",N,N],[11,"new","hrx","Create an empty archive with the specified boundary length.",0,[[["nonzerousize"]],["hrxarchive"]]],[11,"boundary_length","","Get the current boundary length, i.e. the amount of `=` characters in the boundary.",0,[[["self"]],["nonzerousize"]]],[11,"set_boundary_length","","Set new boundary length, if valid.",0,[[["self"],["nonzerousize"]],["result",["hrxerror"]]]],[11,"validate_content","","Validate that no `body`s contain a `boundary` or error out with the path to the first one that does,",0,[[["self"]],["result",["hrxerror"]]]],[11,"into_inner","","Unwraps the contained path.",6,[[["self"]],["string"]]],[11,"into","","",0,[[["self"]],["u"]]],[11,"to_owned","","",0,[[["self"]],["t"]]],[11,"clone_into","","",0,N],[11,"from","","",0,[[["t"]],["t"]]],[11,"try_from","","",0,[[["u"]],["result"]]],[11,"borrow","","",0,[[["self"]],["t"]]],[11,"get_type_id","","",0,[[["self"]],["typeid"]]],[11,"try_into","","",0,[[["self"]],["result"]]],[11,"borrow_mut","","",0,[[["self"]],["t"]]],[11,"into","","",1,[[["self"]],["u"]]],[11,"to_owned","","",1,[[["self"]],["t"]]],[11,"clone_into","","",1,N],[11,"from","","",1,[[["t"]],["t"]]],[11,"try_from","","",1,[[["u"]],["result"]]],[11,"borrow","","",1,[[["self"]],["t"]]],[11,"get_type_id","","",1,[[["self"]],["typeid"]]],[11,"try_into","","",1,[[["self"]],["result"]]],[11,"borrow_mut","","",1,[[["self"]],["t"]]],[11,"into","","",6,[[["self"]],["u"]]],[11,"to_string","","",6,[[["self"]],["string"]]],[11,"to_owned","","",6,[[["self"]],["t"]]],[11,"clone_into","","",6,N],[11,"from","","",6,[[["t"]],["t"]]],[11,"try_from","","",6,[[["u"]],["result"]]],[11,"borrow","","",6,[[["self"]],["t"]]],[11,"get_type_id","","",6,[[["self"]],["typeid"]]],[11,"try_into","","",6,[[["self"]],["result"]]],[11,"borrow_mut","","",6,[[["self"]],["t"]]],[11,"into","","",2,[[["self"]],["u"]]],[11,"to_owned","","",2,[[["self"]],["t"]]],[11,"clone_into","","",2,N],[11,"from","","",2,[[["t"]],["t"]]],[11,"try_from","","",2,[[["u"]],["result"]]],[11,"borrow","","",2,[[["self"]],["t"]]],[11,"get_type_id","","",2,[[["self"]],["typeid"]]],[11,"try_into","","",2,[[["self"]],["result"]]],[11,"borrow_mut","","",2,[[["self"]],["t"]]],[11,"into","","",3,[[["self"]],["u"]]],[11,"to_string","","",3,[[["self"]],["string"]]],[11,"to_owned","","",3,[[["self"]],["t"]]],[11,"clone_into","","",3,N],[11,"from","","",3,[[["t"]],["t"]]],[11,"try_from","","",3,[[["u"]],["result"]]],[11,"borrow","","",3,[[["self"]],["t"]]],[11,"get_type_id","","",3,[[["self"]],["typeid"]]],[11,"try_into","","",3,[[["self"]],["result"]]],[11,"borrow_mut","","",3,[[["self"]],["t"]]],[11,"into","","",4,[[["self"]],["u"]]],[11,"to_owned","","",4,[[["self"]],["t"]]],[11,"clone_into","","",4,N],[11,"from","","",4,[[["t"]],["t"]]],[11,"try_from","","",4,[[["u"]],["result"]]],[11,"borrow","","",4,[[["self"]],["t"]]],[11,"get_type_id","","",4,[[["self"]],["typeid"]]],[11,"try_into","","",4,[[["self"]],["result"]]],[11,"borrow_mut","","",4,[[["self"]],["t"]]],[11,"into","hrx::parse","",5,[[["self"]],["u"]]],[11,"to_string","","",5,[[["self"]],["string"]]],[11,"to_owned","","",5,[[["self"]],["t"]]],[11,"clone_into","","",5,N],[11,"from","","",5,[[["t"]],["t"]]],[11,"try_from","","",5,[[["u"]],["result"]]],[11,"borrow","","",5,[[["self"]],["t"]]],[11,"get_type_id","","",5,[[["self"]],["typeid"]]],[11,"try_into","","",5,[[["self"]],["result"]]],[11,"borrow_mut","","",5,[[["self"]],["t"]]],[11,"partial_cmp","hrx","",0,[[["self"],["hrxarchive"]],["option",["ordering"]]]],[11,"lt","","",0,[[["self"],["hrxarchive"]],["bool"]]],[11,"le","","",0,[[["self"],["hrxarchive"]],["bool"]]],[11,"gt","","",0,[[["self"],["hrxarchive"]],["bool"]]],[11,"ge","","",0,[[["self"],["hrxarchive"]],["bool"]]],[11,"partial_cmp","","",1,[[["self"],["hrxentry"]],["option",["ordering"]]]],[11,"lt","","",1,[[["self"],["hrxentry"]],["bool"]]],[11,"le","","",1,[[["self"],["hrxentry"]],["bool"]]],[11,"gt","","",1,[[["self"],["hrxentry"]],["bool"]]],[11,"ge","","",1,[[["self"],["hrxentry"]],["bool"]]],[11,"partial_cmp","","",4,[[["self"],["hrxentrydata"]],["option",["ordering"]]]],[11,"lt","","",4,[[["self"],["hrxentrydata"]],["bool"]]],[11,"le","","",4,[[["self"],["hrxentrydata"]],["bool"]]],[11,"gt","","",4,[[["self"],["hrxentrydata"]],["bool"]]],[11,"ge","","",4,[[["self"],["hrxentrydata"]],["bool"]]],[11,"partial_cmp","","",6,[[["self"],["hrxpath"]],["option",["ordering"]]]],[11,"lt","","",6,[[["self"],["hrxpath"]],["bool"]]],[11,"le","","",6,[[["self"],["hrxpath"]],["bool"]]],[11,"gt","","",6,[[["self"],["hrxpath"]],["bool"]]],[11,"ge","","",6,[[["self"],["hrxpath"]],["bool"]]],[11,"partial_cmp","","",2,[[["self"],["erroneousbodypath"]],["option",["ordering"]]]],[11,"lt","","",2,[[["self"],["erroneousbodypath"]],["bool"]]],[11,"le","","",2,[[["self"],["erroneousbodypath"]],["bool"]]],[11,"gt","","",2,[[["self"],["erroneousbodypath"]],["bool"]]],[11,"ge","","",2,[[["self"],["erroneousbodypath"]],["bool"]]],[11,"cmp","","",0,[[["self"],["hrxarchive"]],["ordering"]]],[11,"cmp","","",1,[[["self"],["hrxentry"]],["ordering"]]],[11,"cmp","","",4,[[["self"],["hrxentrydata"]],["ordering"]]],[11,"cmp","","",6,[[["self"],["hrxpath"]],["ordering"]]],[11,"cmp","","",2,[[["self"],["erroneousbodypath"]],["ordering"]]],[11,"eq","hrx::parse","",5,[[["self"],["parseerror"]],["bool"]]],[11,"ne","","",5,[[["self"],["parseerror"]],["bool"]]],[11,"eq","hrx","",0,[[["self"],["hrxarchive"]],["bool"]]],[11,"ne","","",0,[[["self"],["hrxarchive"]],["bool"]]],[11,"eq","","",1,[[["self"],["hrxentry"]],["bool"]]],[11,"ne","","",1,[[["self"],["hrxentry"]],["bool"]]],[11,"eq","","",4,[[["self"],["hrxentrydata"]],["bool"]]],[11,"ne","","",4,[[["self"],["hrxentrydata"]],["bool"]]],[11,"eq","","",6,[[["self"],["hrxpath"]],["bool"]]],[11,"ne","","",6,[[["self"],["hrxpath"]],["bool"]]],[11,"eq","","",3,[[["self"],["hrxerror"]],["bool"]]],[11,"ne","","",3,[[["self"],["hrxerror"]],["bool"]]],[11,"eq","","",2,[[["self"],["erroneousbodypath"]],["bool"]]],[11,"ne","","",2,[[["self"],["erroneousbodypath"]],["bool"]]],[11,"from","","",3,[[["parseerror"]],["hrxerror"]]],[11,"from","","",3,[[["erroneousbodypath"]],["hrxerror"]]],[11,"clone","hrx::parse","",5,[[["self"]],["parseerror"]]],[11,"clone","hrx","",0,[[["self"]],["hrxarchive"]]],[11,"clone","","",1,[[["self"]],["hrxentry"]]],[11,"clone","","",4,[[["self"]],["hrxentrydata"]]],[11,"clone","","",6,[[["self"]],["hrxpath"]]],[11,"clone","","",3,[[["self"]],["hrxerror"]]],[11,"clone","","",2,[[["self"]],["erroneousbodypath"]]],[11,"fmt","hrx::parse","",5,[[["self"],["formatter"]],["result",["error"]]]],[11,"fmt","hrx","",6,[[["self"],["formatter"]],["result"]]],[11,"fmt","","",3,[[["self"],["formatter"]],["result"]]],[11,"fmt","hrx::parse","",5,[[["self"],["formatter"]],["result"]]],[11,"fmt","hrx","",0,[[["self"],["formatter"]],["result"]]],[11,"fmt","","",1,[[["self"],["formatter"]],["result"]]],[11,"fmt","","",4,[[["self"],["formatter"]],["result"]]],[11,"fmt","","",6,[[["self"],["formatter"]],["result"]]],[11,"fmt","","",3,[[["self"],["formatter"]],["result"]]],[11,"fmt","","",2,[[["self"],["formatter"]],["result"]]],[11,"hash","","",0,N],[11,"hash","","",1,N],[11,"hash","","",4,N],[11,"hash","","",6,N],[11,"hash","","",2,N],[11,"from_str","","",0,[[["str"]],["result"]]],[11,"from_str","","",6,[[["str"]],["result"]]],[11,"borrow","","",6,[[["self"]],["str"]]],[11,"description","hrx::parse","",5,[[["self"]],["str"]]],[11,"source","hrx","",3,[[["self"]],["option",["error"]]]]],"paths":[[3,"HrxArchive"],[3,"HrxEntry"],[4,"ErroneousBodyPath"],[4,"HrxError"],[4,"HrxEntryData"],[3,"ParseError"],[3,"HrxPath"]]};
searchIndex["jetscii"]={"doc":"A tiny library to efficiently search strings for sets of ASCII characters or byte slices for sets of bytes.","items":[[3,"Bytes","jetscii","Searches a slice for a set of bytes. Up to 16 bytes may be used.",N,N],[3,"AsciiChars","","Searches a string for a set of ASCII characters. Up to 16 characters may be used.",N,N],[3,"ByteSubstring","","Searches a slice for the first occurence of the subslice.",N,N],[3,"Substring","","Searches a string for the first occurence of the substring.",N,N],[6,"BytesConst","","A convenience type that can be used in a constant or static.",N,N],[6,"AsciiCharsConst","","A convenience type that can be used in a constant or static.",N,N],[6,"ByteSubstringConst","","A convenience type that can be used in a constant or static.",N,N],[6,"SubstringConst","","A convenience type that can be used in a constant or static.",N,N],[11,"new","","Manual constructor; prefer using [`bytes!`] instead.",0,N],[11,"find","","Searches the slice for the first matching byte in the set.",0,N],[11,"new","","Manual constructor; prefer using [`ascii_chars!`] instead.",1,N],[11,"find","","Searches the string for the first matching ASCII byte in the set.",1,[[["self"],["str"]],["option",["usize"]]]],[11,"new","","",2,N],[11,"find","","Searches the slice for the first occurence of the subslice.",2,N],[11,"new","","",3,[[["str"]],["self"]]],[11,"find","","Searches the string for the first occurence of the substring.",3,[[["self"],["str"]],["option",["usize"]]]],[14,"bytes","","A convenience constructor for a [`Bytes`] that automatically implements a fallback. Provide 1 to 16 characters.",N,N],[14,"ascii_chars","","A convenience constructor for an [`AsciiChars`] that automatically implements a fallback. Provide 1 to 16 characters.",N,N],[11,"into","","",0,[[["self"]],["u"]]],[11,"from","","",0,[[["t"]],["t"]]],[11,"try_from","","",0,[[["u"]],["result"]]],[11,"borrow","","",0,[[["self"]],["t"]]],[11,"get_type_id","","",0,[[["self"]],["typeid"]]],[11,"try_into","","",0,[[["self"]],["result"]]],[11,"borrow_mut","","",0,[[["self"]],["t"]]],[11,"into","","",1,[[["self"]],["u"]]],[11,"from","","",1,[[["t"]],["t"]]],[11,"try_from","","",1,[[["u"]],["result"]]],[11,"borrow","","",1,[[["self"]],["t"]]],[11,"get_type_id","","",1,[[["self"]],["typeid"]]],[11,"try_into","","",1,[[["self"]],["result"]]],[11,"borrow_mut","","",1,[[["self"]],["t"]]],[11,"into","","",2,[[["self"]],["u"]]],[11,"from","","",2,[[["t"]],["t"]]],[11,"try_from","","",2,[[["u"]],["result"]]],[11,"borrow","","",2,[[["self"]],["t"]]],[11,"get_type_id","","",2,[[["self"]],["typeid"]]],[11,"try_into","","",2,[[["self"]],["result"]]],[11,"borrow_mut","","",2,[[["self"]],["t"]]],[11,"into","","",3,[[["self"]],["u"]]],[11,"from","","",3,[[["t"]],["t"]]],[11,"try_from","","",3,[[["u"]],["result"]]],[11,"borrow","","",3,[[["self"]],["t"]]],[11,"get_type_id","","",3,[[["self"]],["typeid"]]],[11,"try_into","","",3,[[["self"]],["result"]]],[11,"borrow_mut","","",3,[[["self"]],["t"]]]],"paths":[[3,"Bytes"],[3,"AsciiChars"],[3,"ByteSubstring"],[3,"Substring"]]};
searchIndex["lazysort"]={"doc":"","items":[[3,"LazySortIterator","lazysort","",N,N],[3,"LazySortIteratorPartialFirst","","",N,N],[3,"LazySortIteratorPartialLast","","",N,N],[3,"LazySortIteratorBy","","",N,N],[8,"Sorted","","",N,N],[16,"Item","","",0,N],[10,"sorted","","",0,[[["self"]],["lazysortiterator"]]],[8,"SortedPartial","","",N,N],[16,"Item","","",1,N],[10,"sorted_partial_first","","",1,[[["self"]],["lazysortiteratorpartialfirst"]]],[10,"sorted_partial_last","","",1,[[["self"]],["lazysortiteratorpartiallast"]]],[8,"SortedBy","","",N,N],[16,"Item","","",2,N],[10,"sorted_by","","",2,[[["self"],["f"]],["lazysortiteratorby"]]],[11,"sorted","","",3,[[["self"]],["lazysortiterator"]]],[11,"sorted_partial_first","","",3,[[["self"]],["lazysortiteratorpartialfirst"]]],[11,"sorted_partial_last","","",3,[[["self"]],["lazysortiteratorpartiallast"]]],[11,"sorted_by","","",3,[[["self"],["f"]],["lazysortiteratorby"]]],[11,"into","","",3,[[["self"]],["u"]]],[11,"from","","",3,[[["t"]],["t"]]],[11,"into_iter","","",3,[[["self"]],["i"]]],[11,"try_from","","",3,[[["u"]],["result"]]],[11,"borrow","","",3,[[["self"]],["t"]]],[11,"get_type_id","","",3,[[["self"]],["typeid"]]],[11,"try_into","","",3,[[["self"]],["result"]]],[11,"borrow_mut","","",3,[[["self"]],["t"]]],[11,"sorted","","",4,[[["self"]],["lazysortiterator"]]],[11,"sorted_partial_first","","",4,[[["self"]],["lazysortiteratorpartialfirst"]]],[11,"sorted_partial_last","","",4,[[["self"]],["lazysortiteratorpartiallast"]]],[11,"sorted_by","","",4,[[["self"],["f"]],["lazysortiteratorby"]]],[11,"into","","",4,[[["self"]],["u"]]],[11,"from","","",4,[[["t"]],["t"]]],[11,"into_iter","","",4,[[["self"]],["i"]]],[11,"try_from","","",4,[[["u"]],["result"]]],[11,"borrow","","",4,[[["self"]],["t"]]],[11,"get_type_id","","",4,[[["self"]],["typeid"]]],[11,"try_into","","",4,[[["self"]],["result"]]],[11,"borrow_mut","","",4,[[["self"]],["t"]]],[11,"sorted","","",5,[[["self"]],["lazysortiterator"]]],[11,"sorted_partial_first","","",5,[[["self"]],["lazysortiteratorpartialfirst"]]],[11,"sorted_partial_last","","",5,[[["self"]],["lazysortiteratorpartiallast"]]],[11,"sorted_by","","",5,[[["self"],["f"]],["lazysortiteratorby"]]],[11,"into","","",5,[[["self"]],["u"]]],[11,"from","","",5,[[["t"]],["t"]]],[11,"into_iter","","",5,[[["self"]],["i"]]],[11,"try_from","","",5,[[["u"]],["result"]]],[11,"borrow","","",5,[[["self"]],["t"]]],[11,"get_type_id","","",5,[[["self"]],["typeid"]]],[11,"try_into","","",5,[[["self"]],["result"]]],[11,"borrow_mut","","",5,[[["self"]],["t"]]],[11,"sorted","","",6,[[["self"]],["lazysortiterator"]]],[11,"sorted_partial_first","","",6,[[["self"]],["lazysortiteratorpartialfirst"]]],[11,"sorted_partial_last","","",6,[[["self"]],["lazysortiteratorpartiallast"]]],[11,"sorted_by","","",6,[[["self"],["f"]],["lazysortiteratorby"]]],[11,"into","","",6,[[["self"]],["u"]]],[11,"from","","",6,[[["t"]],["t"]]],[11,"into_iter","","",6,[[["self"]],["i"]]],[11,"try_from","","",6,[[["u"]],["result"]]],[11,"borrow","","",6,[[["self"]],["t"]]],[11,"get_type_id","","",6,[[["self"]],["typeid"]]],[11,"try_into","","",6,[[["self"]],["result"]]],[11,"borrow_mut","","",6,[[["self"]],["t"]]],[11,"next","","",3,[[["self"]],["option"]]],[11,"size_hint","","",3,N],[11,"next","","",4,[[["self"]],["option"]]],[11,"size_hint","","",4,N],[11,"next","","",5,[[["self"]],["option"]]],[11,"size_hint","","",5,N],[11,"next","","",6,[[["self"]],["option"]]],[11,"size_hint","","",6,N]],"paths":[[8,"Sorted"],[8,"SortedPartial"],[8,"SortedBy"],[3,"LazySortIterator"],[3,"LazySortIteratorPartialFirst"],[3,"LazySortIteratorPartialLast"],[3,"LazySortIteratorBy"]]};
searchIndex["linked_hash_map"]={"doc":"A `HashMap` wrapper that holds key-value pairs in insertion order.","items":[[3,"LinkedHashMap","linked_hash_map","A linked hash map.",N,N],[3,"Iter","","An insertion-order iterator over a `LinkedHashMap`'s entries, with immutable references to the values.",N,N],[3,"IterMut","","An insertion-order iterator over a `LinkedHashMap`'s entries, with mutable references to the values.",N,N],[3,"IntoIter","","A consuming insertion-order iterator over a `LinkedHashMap`'s entries.",N,N],[3,"Entries","","An insertion-order iterator over a `LinkedHashMap`'s entries represented as an `OccupiedEntry`.",N,N],[3,"Keys","","An insertion-order iterator over a `LinkedHashMap`'s keys.",N,N],[3,"Values","","An insertion-order iterator over a `LinkedHashMap`'s values.",N,N],[3,"OccupiedEntry","","A view into a single occupied location in a `LinkedHashMap`.",N,N],[3,"VacantEntry","","A view into a single empty location in a `LinkedHashMap`.",N,N],[4,"Entry","","A view into a single location in a map, which may be vacant or occupied.",N,N],[13,"Occupied","","An occupied Entry.",0,N],[13,"Vacant","","A vacant Entry.",0,N],[11,"new","","Creates a linked hash map.",1,[[],["self"]]],[11,"with_capacity","","Creates an empty linked hash map with the given initial capacity.",1,[[["usize"]],["self"]]],[11,"with_hasher","","Creates an empty linked hash map with the given initial hash builder.",1,[[["s"]],["self"]]],[11,"with_capacity_and_hasher","","Creates an empty linked hash map with the given initial capacity and hash builder.",1,[[["usize"],["s"]],["self"]]],[11,"reserve","","Reserves capacity for at least `additional` more elements to be inserted into the map. The map may reserve more space to avoid frequent allocations.",1,[[["self"],["usize"]]]],[11,"shrink_to_fit","","Shrinks the capacity of the map as much as possible. It will drop down as much as possible while maintaining the internal rules and possibly leaving some space in accordance with the resize policy.",1,[[["self"]]]],[11,"entry","","Gets the given key's corresponding entry in the map for in-place manipulation.",1,[[["self"],["k"]],["entry"]]],[11,"entries","","Returns an iterator visiting all entries in insertion order. Iterator element type is `OccupiedEntry<K, V, S>`. Allows for removal as well as replacing the entry.",1,[[["self"]],["entries"]]],[11,"insert","","Inserts a key-value pair into the map. If the key already existed, the old value is returned.",1,[[["self"],["k"],["v"]],["option"]]],[11,"contains_key","","Checks if the map contains the given key.",1,[[["self"],["q"]],["bool"]]],[11,"get","","Returns the value corresponding to the key in the map.",1,[[["self"],["q"]],["option"]]],[11,"get_mut","","Returns the mutable reference corresponding to the key in the map.",1,[[["self"],["q"]],["option"]]],[11,"get_refresh","","Returns the value corresponding to the key in the map.",1,[[["self"],["q"]],["option"]]],[11,"remove","","Removes and returns the value corresponding to the key from the map.",1,[[["self"],["q"]],["option"]]],[11,"capacity","","Returns the maximum number of key-value pairs the map can hold without reallocating.",1,[[["self"]],["usize"]]],[11,"pop_front","","Removes the first entry.",1,[[["self"]],["option"]]],[11,"front","","Gets the first entry.",1,[[["self"]],["option"]]],[11,"pop_back","","Removes the last entry.",1,[[["self"]],["option"]]],[11,"back","","Gets the last entry.",1,[[["self"]],["option"]]],[11,"len","","Returns the number of key-value pairs in the map.",1,[[["self"]],["usize"]]],[11,"is_empty","","Returns whether the map is currently empty.",1,[[["self"]],["bool"]]],[11,"hasher","","Returns a reference to the map's hasher.",1,[[["self"]],["s"]]],[11,"clear","","Clears the map of all key-value pairs.",1,[[["self"]]]],[11,"iter","","Returns a double-ended iterator visiting all key-value pairs in order of insertion. Iterator element type is `(&'a K, &'a V)`",1,[[["self"]],["iter"]]],[11,"iter_mut","","Returns a double-ended iterator visiting all key-value pairs in order of insertion. Iterator element type is `(&'a K, &'a mut V)` # Examples ``` use linked_hash_map::LinkedHashMap;",1,[[["self"]],["itermut"]]],[11,"keys","","Returns a double-ended iterator visiting all key in order of insertion.",1,[[["self"]],["keys"]]],[11,"values","","Returns a double-ended iterator visiting all values in order of insertion.",1,[[["self"]],["values"]]],[11,"key","","Returns the entry key",0,[[["self"]],["k"]]],[11,"or_insert","","Ensures a value is in the entry by inserting the default if empty, and returns a mutable reference to the value in the entry.",0,[[["self"],["v"]],["v"]]],[11,"or_insert_with","","Ensures a value is in the entry by inserting the result of the default function if empty, and returns a mutable reference to the value in the entry.",0,[[["self"],["f"]],["v"]]],[11,"key","","Gets a reference to the entry key",2,[[["self"]],["k"]]],[11,"get","","Gets a reference to the value in the entry.",2,[[["self"]],["v"]]],[11,"get_mut","","Gets a mutable reference to the value in the entry.",2,[[["self"]],["v"]]],[11,"into_mut","","Converts the OccupiedEntry into a mutable reference to the value in the entry with a lifetime bound to the map itself",2,[[["self"]],["v"]]],[11,"insert","","Sets the value of the entry, and returns the entry's old value",2,[[["self"],["v"]],["v"]]],[11,"remove","","Takes the value out of the entry, and returns it",2,[[["self"]],["v"]]],[11,"key","","Gets a reference to the entry key",3,[[["self"]],["k"]]],[11,"insert","","Sets the value of the entry with the VacantEntry's key, and returns a mutable reference to it",3,[[["self"],["v"]],["v"]]],[11,"into","","",1,[[["self"]],["u"]]],[11,"to_owned","","",1,[[["self"]],["t"]]],[11,"clone_into","","",1,N],[11,"from","","",1,[[["t"]],["t"]]],[11,"into_iter","","",1,[[["self"]],["i"]]],[11,"try_from","","",1,[[["u"]],["result"]]],[11,"borrow","","",1,[[["self"]],["t"]]],[11,"get_type_id","","",1,[[["self"]],["typeid"]]],[11,"try_into","","",1,[[["self"]],["result"]]],[11,"borrow_mut","","",1,[[["self"]],["t"]]],[11,"into","","",4,[[["self"]],["u"]]],[11,"to_owned","","",4,[[["self"]],["t"]]],[11,"clone_into","","",4,N],[11,"from","","",4,[[["t"]],["t"]]],[11,"into_iter","","",4,[[["self"]],["i"]]],[11,"try_from","","",4,[[["u"]],["result"]]],[11,"borrow","","",4,[[["self"]],["t"]]],[11,"get_type_id","","",4,[[["self"]],["typeid"]]],[11,"try_into","","",4,[[["self"]],["result"]]],[11,"borrow_mut","","",4,[[["self"]],["t"]]],[11,"into","","",5,[[["self"]],["u"]]],[11,"from","","",5,[[["t"]],["t"]]],[11,"into_iter","","",5,[[["self"]],["i"]]],[11,"try_from","","",5,[[["u"]],["result"]]],[11,"borrow","","",5,[[["self"]],["t"]]],[11,"get_type_id","","",5,[[["self"]],["typeid"]]],[11,"try_into","","",5,[[["self"]],["result"]]],[11,"borrow_mut","","",5,[[["self"]],["t"]]],[11,"into","","",6,[[["self"]],["u"]]],[11,"to_owned","","",6,[[["self"]],["t"]]],[11,"clone_into","","",6,N],[11,"from","","",6,[[["t"]],["t"]]],[11,"into_iter","","",6,[[["self"]],["i"]]],[11,"try_from","","",6,[[["u"]],["result"]]],[11,"borrow","","",6,[[["self"]],["t"]]],[11,"get_type_id","","",6,[[["self"]],["typeid"]]],[11,"try_into","","",6,[[["self"]],["result"]]],[11,"borrow_mut","","",6,[[["self"]],["t"]]],[11,"into","","",7,[[["self"]],["u"]]],[11,"from","","",7,[[["t"]],["t"]]],[11,"into_iter","","",7,[[["self"]],["i"]]],[11,"try_from","","",7,[[["u"]],["result"]]],[11,"borrow","","",7,[[["self"]],["t"]]],[11,"get_type_id","","",7,[[["self"]],["typeid"]]],[11,"try_into","","",7,[[["self"]],["result"]]],[11,"borrow_mut","","",7,[[["self"]],["t"]]],[11,"into","","",8,[[["self"]],["u"]]],[11,"to_owned","","",8,[[["self"]],["t"]]],[11,"clone_into","","",8,N],[11,"from","","",8,[[["t"]],["t"]]],[11,"into_iter","","",8,[[["self"]],["i"]]],[11,"try_from","","",8,[[["u"]],["result"]]],[11,"borrow","","",8,[[["self"]],["t"]]],[11,"get_type_id","","",8,[[["self"]],["typeid"]]],[11,"try_into","","",8,[[["self"]],["result"]]],[11,"borrow_mut","","",8,[[["self"]],["t"]]],[11,"into","","",9,[[["self"]],["u"]]],[11,"to_owned","","",9,[[["self"]],["t"]]],[11,"clone_into","","",9,N],[11,"from","","",9,[[["t"]],["t"]]],[11,"into_iter","","",9,[[["self"]],["i"]]],[11,"try_from","","",9,[[["u"]],["result"]]],[11,"borrow","","",9,[[["self"]],["t"]]],[11,"get_type_id","","",9,[[["self"]],["typeid"]]],[11,"try_into","","",9,[[["self"]],["result"]]],[11,"borrow_mut","","",9,[[["self"]],["t"]]],[11,"into","","",2,[[["self"]],["u"]]],[11,"from","","",2,[[["t"]],["t"]]],[11,"try_from","","",2,[[["u"]],["result"]]],[11,"borrow","","",2,[[["self"]],["t"]]],[11,"get_type_id","","",2,[[["self"]],["typeid"]]],[11,"try_into","","",2,[[["self"]],["result"]]],[11,"borrow_mut","","",2,[[["self"]],["t"]]],[11,"into","","",3,[[["self"]],["u"]]],[11,"from","","",3,[[["t"]],["t"]]],[11,"try_from","","",3,[[["u"]],["result"]]],[11,"borrow","","",3,[[["self"]],["t"]]],[11,"get_type_id","","",3,[[["self"]],["typeid"]]],[11,"try_into","","",3,[[["self"]],["result"]]],[11,"borrow_mut","","",3,[[["self"]],["t"]]],[11,"into","","",0,[[["self"]],["u"]]],[11,"from","","",0,[[["t"]],["t"]]],[11,"try_from","","",0,[[["u"]],["result"]]],[11,"borrow","","",0,[[["self"]],["t"]]],[11,"get_type_id","","",0,[[["self"]],["typeid"]]],[11,"try_into","","",0,[[["self"]],["result"]]],[11,"borrow_mut","","",0,[[["self"]],["t"]]],[11,"next","","",4,[[["self"]],["option"]]],[11,"size_hint","","",4,N],[11,"next","","",5,[[["self"]],["option"]]],[11,"size_hint","","",5,N],[11,"next","","",6,[[["self"]],["option"]]],[11,"size_hint","","",6,N],[11,"next","","",7,[[["self"]],["option",["occupiedentry"]]]],[11,"size_hint","","",7,N],[11,"next","","",8,[[["self"]],["option"]]],[11,"size_hint","","",8,N],[11,"next","","",9,[[["self"]],["option"]]],[11,"size_hint","","",9,N],[11,"len","","",4,[[["self"]],["usize"]]],[11,"len","","",5,[[["self"]],["usize"]]],[11,"len","","",6,[[["self"]],["usize"]]],[11,"len","","",8,[[["self"]],["usize"]]],[11,"len","","",9,[[["self"]],["usize"]]],[11,"partial_cmp","","",1,[[["self"],["self"]],["option",["ordering"]]]],[11,"lt","","",1,[[["self"],["self"]],["bool"]]],[11,"le","","",1,[[["self"],["self"]],["bool"]]],[11,"ge","","",1,[[["self"],["self"]],["bool"]]],[11,"gt","","",1,[[["self"],["self"]],["bool"]]],[11,"default","","",1,[[],["self"]]],[11,"next_back","","",4,[[["self"]],["option"]]],[11,"next_back","","",5,[[["self"]],["option"]]],[11,"next_back","","",6,[[["self"]],["option"]]],[11,"next_back","","",8,[[["self"]],["option"]]],[11,"next_back","","",9,[[["self"]],["option"]]],[11,"cmp","","",1,[[["self"],["self"]],["ordering"]]],[11,"eq","","",1,[[["self"],["self"]],["bool"]]],[11,"clone","","",1,[[["self"]],["self"]]],[11,"clone","","",4,[[["self"]],["self"]]],[11,"clone","","",6,[[["self"]],["self"]]],[11,"clone","","",8,[[["self"]],["self"]]],[11,"clone","","",9,[[["self"]],["self"]]],[11,"into_iter","","",1,[[["self"]],["intoiter"]]],[11,"extend","","",1,[[["self"],["i"]]]],[11,"extend","","",1,[[["self"],["i"]]]],[11,"drop","","",1,[[["self"]]]],[11,"drop","","",6,[[["self"]]]],[11,"fmt","","Returns a string that lists the key-value pairs in insertion order.",1,[[["self"],["formatter"]],["result"]]],[11,"hash","","",1,[[["self"],["h"]]]],[11,"index","","",1,[[["self"],["q"]],["v"]]],[11,"index_mut","","",1,[[["self"],["q"]],["v"]]],[11,"from_iter","","",1,[[["i"]],["self"]]]],"paths":[[4,"Entry"],[3,"LinkedHashMap"],[3,"OccupiedEntry"],[3,"VacantEntry"],[3,"Iter"],[3,"IterMut"],[3,"IntoIter"],[3,"Entries"],[3,"Keys"],[3,"Values"]]};
initSearch(searchIndex);
