#![allow(unused, non_snake_case, non_camel_case_types)]

extern crate libc;

pub const SQLITE_VERSION : &'static str = "3.8.10.1";
pub const SQLITE_VERSION_NUMBER : libc::c_int = 3008010;
pub const SQLITE_SOURCE_ID : &'static str = "2015-05-09 12:14:55 05b4b1f2a937c06c90db70c09890038f6c98ec40";

pub const SQLITE_OK : libc::c_int = 0; /* Successful result */
pub const SQLITE_ERROR : libc::c_int = 1; /* SQL error or missing database */
pub const SQLITE_INTERNAL : libc::c_int = 2; /* Internal logic error in SQLite */
pub const SQLITE_PERM : libc::c_int = 3; /* Access permission denied */
pub const SQLITE_ABORT : libc::c_int = 4; /* Callback routine requested an abort */
pub const SQLITE_BUSY : libc::c_int = 5; /* The database file is locked */
pub const SQLITE_LOCKED : libc::c_int = 6; /* A table in the database is locked */
pub const SQLITE_NOMEM : libc::c_int = 7; /* A malloc() failed */
pub const SQLITE_READONLY : libc::c_int = 8; /* Attempt to write a readonly database */
pub const SQLITE_INTERRUPT : libc::c_int = 9; /* Operation terminated by sqlite3_interrupt()*/
pub const SQLITE_IOERR : libc::c_int = 10; /* Some kind of disk I/O error occurred */
pub const SQLITE_CORRUPT : libc::c_int = 11; /* The database disk image is malformed */
pub const SQLITE_NOTFOUND : libc::c_int = 12; /* Unknown opcode in sqlite3_file_control() */
pub const SQLITE_FULL : libc::c_int = 13; /* Insertion failed because database is full */
pub const SQLITE_CANTOPEN : libc::c_int = 14; /* Unable to open the database file */
pub const SQLITE_PROTOCOL : libc::c_int = 15; /* Database lock protocol error */
pub const SQLITE_EMPTY : libc::c_int = 16; /* Database is empty */
pub const SQLITE_SCHEMA : libc::c_int = 17; /* The database schema changed */
pub const SQLITE_TOOBIG : libc::c_int = 18; /* String or BLOB exceeds size limit */
pub const SQLITE_CONSTRAINT : libc::c_int = 19; /* Abort due to pub constraint violation */
pub const SQLITE_MISMATCH : libc::c_int = 20; /* Data type mismatch */
pub const SQLITE_MISUSE : libc::c_int = 21; /* Library used incorrectly */
pub const SQLITE_NOLFS : libc::c_int = 22; /* Uses OS features not supported on host */
pub const SQLITE_AUTH : libc::c_int = 23; /* Authorization denied */
pub const SQLITE_FORMAT : libc::c_int = 24; /* Auxiliary database format error */
pub const SQLITE_RANGE : libc::c_int = 25; /* 2nd parameter to sqlite3_bind out of range */
pub const SQLITE_NOTADB : libc::c_int = 26; /* File opened that is not a database file */
pub const SQLITE_NOTICE : libc::c_int = 27; /* Notifications from sqlite3_log() */
pub const SQLITE_WARNING : libc::c_int = 28; /* Warnings from sqlite3_log() */
pub const SQLITE_ROW : libc::c_int = 100; /* sqlite3_step() has another row ready */
pub const SQLITE_DONE : libc::c_int = 101; /* sqlite3_step() has finished executing */

pub const SQLITE_IOERR_READ : libc::c_int = (SQLITE_IOERR | (1<<8));
pub const SQLITE_IOERR_SHORT_READ : libc::c_int = (SQLITE_IOERR | (2<<8));
pub const SQLITE_IOERR_WRITE : libc::c_int = (SQLITE_IOERR | (3<<8));
pub const SQLITE_IOERR_FSYNC : libc::c_int = (SQLITE_IOERR | (4<<8));
pub const SQLITE_IOERR_DIR_FSYNC : libc::c_int = (SQLITE_IOERR | (5<<8));
pub const SQLITE_IOERR_TRUNCATE : libc::c_int = (SQLITE_IOERR | (6<<8));
pub const SQLITE_IOERR_FSTAT : libc::c_int = (SQLITE_IOERR | (7<<8));
pub const SQLITE_IOERR_UNLOCK : libc::c_int = (SQLITE_IOERR | (8<<8));
pub const SQLITE_IOERR_RDLOCK : libc::c_int = (SQLITE_IOERR | (9<<8));
pub const SQLITE_IOERR_DELETE : libc::c_int = (SQLITE_IOERR | (10<<8));
pub const SQLITE_IOERR_BLOCKED : libc::c_int = (SQLITE_IOERR | (11<<8));
pub const SQLITE_IOERR_NOMEM : libc::c_int = (SQLITE_IOERR | (12<<8));
pub const SQLITE_IOERR_ACCESS : libc::c_int = (SQLITE_IOERR | (13<<8));
pub const SQLITE_IOERR_CHECKRESERVEDLOCK : libc::c_int = (SQLITE_IOERR | (14<<8));
pub const SQLITE_IOERR_LOCK : libc::c_int = (SQLITE_IOERR | (15<<8));
pub const SQLITE_IOERR_CLOSE : libc::c_int = (SQLITE_IOERR | (16<<8));
pub const SQLITE_IOERR_DIR_CLOSE : libc::c_int = (SQLITE_IOERR | (17<<8));
pub const SQLITE_IOERR_SHMOPEN : libc::c_int = (SQLITE_IOERR | (18<<8));
pub const SQLITE_IOERR_SHMSIZE : libc::c_int = (SQLITE_IOERR | (19<<8));
pub const SQLITE_IOERR_SHMLOCK : libc::c_int = (SQLITE_IOERR | (20<<8));
pub const SQLITE_IOERR_SHMMAP : libc::c_int = (SQLITE_IOERR | (21<<8));
pub const SQLITE_IOERR_SEEK : libc::c_int = (SQLITE_IOERR | (22<<8));
pub const SQLITE_IOERR_DELETE_NOENT : libc::c_int = (SQLITE_IOERR | (23<<8));
pub const SQLITE_IOERR_MMAP : libc::c_int = (SQLITE_IOERR | (24<<8));
pub const SQLITE_IOERR_GETTEMPPATH : libc::c_int = (SQLITE_IOERR | (25<<8));
pub const SQLITE_IOERR_CONVPATH : libc::c_int = (SQLITE_IOERR | (26<<8));
pub const SQLITE_LOCKED_SHAREDCACHE : libc::c_int = (SQLITE_LOCKED |  (1<<8));
pub const SQLITE_BUSY_RECOVERY : libc::c_int = (SQLITE_BUSY   |  (1<<8));
pub const SQLITE_BUSY_SNAPSHOT : libc::c_int = (SQLITE_BUSY   |  (2<<8));
pub const SQLITE_CANTOPEN_NOTEMPDIR : libc::c_int = (SQLITE_CANTOPEN | (1<<8));
pub const SQLITE_CANTOPEN_ISDIR : libc::c_int = (SQLITE_CANTOPEN | (2<<8));
pub const SQLITE_CANTOPEN_FULLPATH : libc::c_int = (SQLITE_CANTOPEN | (3<<8));
pub const SQLITE_CANTOPEN_CONVPATH : libc::c_int = (SQLITE_CANTOPEN | (4<<8));
pub const SQLITE_CORRUPT_VTAB : libc::c_int = (SQLITE_CORRUPT | (1<<8));
pub const SQLITE_READONLY_RECOVERY : libc::c_int = (SQLITE_READONLY | (1<<8));
pub const SQLITE_READONLY_CANTLOCK : libc::c_int = (SQLITE_READONLY | (2<<8));
pub const SQLITE_READONLY_ROLLBACK : libc::c_int = (SQLITE_READONLY | (3<<8));
pub const SQLITE_READONLY_DBMOVED : libc::c_int = (SQLITE_READONLY | (4<<8));
pub const SQLITE_ABORT_ROLLBACK : libc::c_int = (SQLITE_ABORT | (2<<8));
pub const SQLITE_CONSTRAINT_CHECK : libc::c_int = (SQLITE_CONSTRAINT | (1<<8));
pub const SQLITE_CONSTRAINT_COMMITHOOK : libc::c_int = (SQLITE_CONSTRAINT | (2<<8));
pub const SQLITE_CONSTRAINT_FOREIGNKEY : libc::c_int = (SQLITE_CONSTRAINT | (3<<8));
pub const SQLITE_CONSTRAINT_FUNCTION : libc::c_int = (SQLITE_CONSTRAINT | (4<<8));
pub const SQLITE_CONSTRAINT_NOTNULL : libc::c_int = (SQLITE_CONSTRAINT | (5<<8));
pub const SQLITE_CONSTRAINT_PRIMARYKEY : libc::c_int = (SQLITE_CONSTRAINT | (6<<8));
pub const SQLITE_CONSTRAINT_TRIGGER : libc::c_int = (SQLITE_CONSTRAINT | (7<<8));
pub const SQLITE_CONSTRAINT_UNIQUE : libc::c_int = (SQLITE_CONSTRAINT | (8<<8));
pub const SQLITE_CONSTRAINT_VTAB : libc::c_int = (SQLITE_CONSTRAINT | (9<<8));
pub const SQLITE_CONSTRAINT_ROWID : libc::c_int = (SQLITE_CONSTRAINT |(10<<8));
pub const SQLITE_NOTICE_RECOVER_WAL : libc::c_int = (SQLITE_NOTICE | (1<<8));
pub const SQLITE_NOTICE_RECOVER_ROLLBACK : libc::c_int = (SQLITE_NOTICE | (2<<8));
pub const SQLITE_WARNING_AUTOINDEX : libc::c_int = (SQLITE_WARNING | (1<<8));
pub const SQLITE_AUTH_USER : libc::c_int = (SQLITE_AUTH | (1<<8));

pub const SQLITE_OPEN_READONLY : libc::c_int = 0x00000001; /* Ok for sqlite3_open_v2() */
pub const SQLITE_OPEN_READWRITE : libc::c_int = 0x00000002; /* Ok for sqlite3_open_v2() */
pub const SQLITE_OPEN_CREATE : libc::c_int = 0x00000004; /* Ok for sqlite3_open_v2() */
pub const SQLITE_OPEN_DELETEONCLOSE : libc::c_int = 0x00000008; /* VFS only */
pub const SQLITE_OPEN_EXCLUSIVE : libc::c_int = 0x00000010; /* VFS only */
pub const SQLITE_OPEN_AUTOPROXY : libc::c_int = 0x00000020; /* VFS only */
pub const SQLITE_OPEN_URI : libc::c_int = 0x00000040; /* Ok for sqlite3_open_v2() */
pub const SQLITE_OPEN_MEMORY : libc::c_int = 0x00000080; /* Ok for sqlite3_open_v2() */
pub const SQLITE_OPEN_MAIN_DB : libc::c_int = 0x00000100; /* VFS only */
pub const SQLITE_OPEN_TEMP_DB : libc::c_int = 0x00000200; /* VFS only */
pub const SQLITE_OPEN_TRANSIENT_DB : libc::c_int = 0x00000400; /* VFS only */
pub const SQLITE_OPEN_MAIN_JOURNAL : libc::c_int = 0x00000800; /* VFS only */
pub const SQLITE_OPEN_TEMP_JOURNAL : libc::c_int = 0x00001000; /* VFS only */
pub const SQLITE_OPEN_SUBJOURNAL : libc::c_int = 0x00002000; /* VFS only */
pub const SQLITE_OPEN_MASTER_JOURNAL : libc::c_int = 0x00004000; /* VFS only */
pub const SQLITE_OPEN_NOMUTEX : libc::c_int = 0x00008000; /* Ok for sqlite3_open_v2() */
pub const SQLITE_OPEN_FULLMUTEX : libc::c_int = 0x00010000; /* Ok for sqlite3_open_v2() */
pub const SQLITE_OPEN_SHAREDCACHE : libc::c_int = 0x00020000; /* Ok for sqlite3_open_v2() */
pub const SQLITE_OPEN_PRIVATECACHE : libc::c_int = 0x00040000; /* Ok for sqlite3_open_v2() */
pub const SQLITE_OPEN_WAL : libc::c_int = 0x00080000; /* VFS only */
pub const SQLITE_IOCAP_ATOMIC : libc::c_int = 0x00000001; 
pub const SQLITE_IOCAP_ATOMIC512 : libc::c_int = 0x00000002;
pub const SQLITE_IOCAP_ATOMIC1K : libc::c_int = 0x00000004; 
pub const SQLITE_IOCAP_ATOMIC2K : libc::c_int = 0x00000008;
pub const SQLITE_IOCAP_ATOMIC4K : libc::c_int = 0x00000010; 
pub const SQLITE_IOCAP_ATOMIC8K : libc::c_int = 0x00000020;
pub const SQLITE_IOCAP_ATOMIC16K : libc::c_int = 0x00000040; 
pub const SQLITE_IOCAP_ATOMIC32K : libc::c_int = 0x00000080;
pub const SQLITE_IOCAP_ATOMIC64K : libc::c_int = 0x00000100; 
pub const SQLITE_IOCAP_SAFE_APPEND : libc::c_int = 0x00000200;
pub const SQLITE_IOCAP_SEQUENTIAL : libc::c_int = 0x00000400; 
pub const SQLITE_IOCAP_UNDELETABLE_WHEN_OPEN : libc::c_int = 0x00000800;
pub const SQLITE_IOCAP_POWERSAFE_OVERWRITE : libc::c_int = 0x00001000; 
pub const SQLITE_IOCAP_IMMUTABLE : libc::c_int = 0x00002000;
pub const SQLITE_LOCK_NONE : libc::c_int = 0; 
pub const SQLITE_LOCK_SHARED : libc::c_int = 1;
pub const SQLITE_LOCK_RESERVED : libc::c_int = 2; 
pub const SQLITE_LOCK_PENDING : libc::c_int = 3;
pub const SQLITE_LOCK_EXCLUSIVE : libc::c_int = 4; 
pub const SQLITE_SYNC_NORMAL : libc::c_int = 0x00002;
pub const SQLITE_SYNC_FULL : libc::c_int = 0x00003; 
pub const SQLITE_SYNC_DATAONLY : libc::c_int = 0x00010;
pub const SQLITE_FCNTL_LOCKSTATE : libc::c_int = 1; 
pub const SQLITE_FCNTL_GET_LOCKPROXYFILE : libc::c_int = 2;
pub const SQLITE_FCNTL_SET_LOCKPROXYFILE : libc::c_int = 3; 
pub const SQLITE_FCNTL_LAST_ERRNO : libc::c_int = 4;
pub const SQLITE_FCNTL_SIZE_HINT : libc::c_int = 5; 
pub const SQLITE_FCNTL_CHUNK_SIZE : libc::c_int = 6;
pub const SQLITE_FCNTL_FILE_POINTER : libc::c_int = 7; 
pub const SQLITE_FCNTL_SYNC_OMITTED : libc::c_int = 8;
pub const SQLITE_FCNTL_WIN32_AV_RETRY : libc::c_int = 9; 
pub const SQLITE_FCNTL_PERSIST_WAL : libc::c_int = 10;
pub const SQLITE_FCNTL_OVERWRITE : libc::c_int = 11; 
pub const SQLITE_FCNTL_VFSNAME : libc::c_int = 12;
pub const SQLITE_FCNTL_POWERSAFE_OVERWRITE : libc::c_int = 13; 
pub const SQLITE_FCNTL_PRAGMA : libc::c_int = 14;
pub const SQLITE_FCNTL_BUSYHANDLER : libc::c_int = 15; 
pub const SQLITE_FCNTL_TEMPFILENAME : libc::c_int = 16;
pub const SQLITE_FCNTL_MMAP_SIZE : libc::c_int = 18; 
pub const SQLITE_FCNTL_TRACE : libc::c_int = 19;
pub const SQLITE_FCNTL_HAS_MOVED : libc::c_int = 20; 
pub const SQLITE_FCNTL_SYNC : libc::c_int = 21;
pub const SQLITE_FCNTL_COMMIT_PHASETWO : libc::c_int = 22; 
pub const SQLITE_FCNTL_WIN32_SET_HANDLE : libc::c_int = 23;
pub const SQLITE_FCNTL_WAL_BLOCK : libc::c_int = 24; 
pub const SQLITE_GET_LOCKPROXYFILE : libc::c_int = SQLITE_FCNTL_GET_LOCKPROXYFILE;
pub const SQLITE_SET_LOCKPROXYFILE : libc::c_int = SQLITE_FCNTL_SET_LOCKPROXYFILE; 
pub const SQLITE_LAST_ERRNO : libc::c_int = SQLITE_FCNTL_LAST_ERRNO;
pub const SQLITE_ACCESS_EXISTS : libc::c_int = 0; 
pub const SQLITE_ACCESS_READWRITE : libc::c_int = 1;   /* Used by PRAGMA temp_store_directory */
pub const SQLITE_ACCESS_READ : libc::c_int = 2; /* Unused */
pub const SQLITE_SHM_UNLOCK : libc::c_int = 1; 
pub const SQLITE_SHM_LOCK : libc::c_int = 2;
pub const SQLITE_SHM_SHARED : libc::c_int = 4; 
pub const SQLITE_SHM_EXCLUSIVE : libc::c_int = 8;
pub const SQLITE_SHM_NLOCK : libc::c_int = 8; 
pub const SQLITE_CONFIG_SINGLETHREAD : libc::c_int = 1;  /* nil */
pub const SQLITE_CONFIG_MULTITHREAD : libc::c_int = 2; /* nil */
pub const SQLITE_CONFIG_SERIALIZED : libc::c_int = 3; /* nil */
pub const SQLITE_CONFIG_MALLOC : libc::c_int = 4; /* sqlite3_mem_methods* */
pub const SQLITE_CONFIG_GETMALLOC : libc::c_int = 5; /* sqlite3_mem_methods* */
pub const SQLITE_CONFIG_SCRATCH : libc::c_int = 6; /* void*, int sz, int N */
pub const SQLITE_CONFIG_PAGECACHE : libc::c_int = 7; /* void*, int sz, int N */
pub const SQLITE_CONFIG_HEAP : libc::c_int = 8; /* void*, int nByte, int min */
pub const SQLITE_CONFIG_MEMSTATUS : libc::c_int = 9; /* boolean */
pub const SQLITE_CONFIG_MUTEX : libc::c_int = 10; /* sqlite3_mutex_methods* */
pub const SQLITE_CONFIG_GETMUTEX : libc::c_int = 11; /* sqlite3_mutex_methods* */
pub const SQLITE_CONFIG_LOOKASIDE : libc::c_int = 13; /* int int */
pub const SQLITE_CONFIG_PCACHE : libc::c_int = 14; /* no-op */
pub const SQLITE_CONFIG_GETPCACHE : libc::c_int = 15; /* no-op */
pub const SQLITE_CONFIG_LOG : libc::c_int = 16; /* xFunc, void* */
pub const SQLITE_CONFIG_URI : libc::c_int = 17; /* int */
pub const SQLITE_CONFIG_PCACHE2 : libc::c_int = 18; /* sqlite3_pcache_methods2* */
pub const SQLITE_CONFIG_GETPCACHE2 : libc::c_int = 19; /* sqlite3_pcache_methods2* */
pub const SQLITE_CONFIG_COVERING_INDEX_SCAN : libc::c_int = 20; /* int */
pub const SQLITE_CONFIG_SQLLOG : libc::c_int = 21; /* xSqllog, void* */
pub const SQLITE_CONFIG_MMAP_SIZE : libc::c_int = 22; /* sqlite3_int64, sqlite3_int64 */
pub const SQLITE_CONFIG_WIN32_HEAPSIZE : libc::c_int = 23; /* int nByte */
pub const SQLITE_CONFIG_PCACHE_HDRSZ : libc::c_int = 24; /* int *psz */
pub const SQLITE_CONFIG_PMASZ : libc::c_int = 25; /* unsigned int szPma */
pub const SQLITE_DBCONFIG_LOOKASIDE : libc::c_int = 1001; /* void* int int */
pub const SQLITE_DBCONFIG_ENABLE_FKEY : libc::c_int = 1002; /* int int* */
pub const SQLITE_DBCONFIG_ENABLE_TRIGGER : libc::c_int = 1003; /* int int* */
pub const SQLITE_DENY : libc::c_int = 1; /* Abort the SQL statement with an error */
pub const SQLITE_IGNORE : libc::c_int = 2; /* Don't allow access, but don't generate an error */
pub const SQLITE_CREATE_INDEX : libc::c_int = 1; /* Index Name      Table Name      */
pub const SQLITE_CREATE_TABLE : libc::c_int = 2; /* Table Name      NULL            */
pub const SQLITE_CREATE_TEMP_INDEX : libc::c_int = 3; /* Index Name      Table Name      */
pub const SQLITE_CREATE_TEMP_TABLE : libc::c_int = 4; /* Table Name      NULL            */
pub const SQLITE_CREATE_TEMP_TRIGGER : libc::c_int = 5; /* Trigger Name    Table Name      */
pub const SQLITE_CREATE_TEMP_VIEW : libc::c_int = 6; /* View Name       NULL            */
pub const SQLITE_CREATE_TRIGGER : libc::c_int = 7; /* Trigger Name    Table Name      */
pub const SQLITE_CREATE_VIEW : libc::c_int = 8; /* View Name       NULL            */
pub const SQLITE_DELETE : libc::c_int = 9; /* Table Name      NULL            */
pub const SQLITE_DROP_INDEX : libc::c_int = 10; /* Index Name      Table Name      */
pub const SQLITE_DROP_TABLE : libc::c_int = 11; /* Table Name      NULL            */
pub const SQLITE_DROP_TEMP_INDEX : libc::c_int = 12; /* Index Name      Table Name      */
pub const SQLITE_DROP_TEMP_TABLE : libc::c_int = 13; /* Table Name      NULL            */
pub const SQLITE_DROP_TEMP_TRIGGER : libc::c_int = 14; /* Trigger Name    Table Name      */
pub const SQLITE_DROP_TEMP_VIEW : libc::c_int = 15; /* View Name       NULL            */
pub const SQLITE_DROP_TRIGGER : libc::c_int = 16; /* Trigger Name    Table Name      */
pub const SQLITE_DROP_VIEW : libc::c_int = 17; /* View Name       NULL            */
pub const SQLITE_INSERT : libc::c_int = 18; /* Table Name      NULL            */
pub const SQLITE_PRAGMA : libc::c_int = 19; /* Pragma Name     1st arg or NULL */
pub const SQLITE_READ : libc::c_int = 20; /* Table Name      Column Name     */
pub const SQLITE_SELECT : libc::c_int = 21; /* NULL            NULL            */
pub const SQLITE_TRANSACTION : libc::c_int = 22; /* Operation       NULL            */
pub const SQLITE_UPDATE : libc::c_int = 23; /* Table Name      Column Name     */
pub const SQLITE_ATTACH : libc::c_int = 24; /* Filename        NULL            */
pub const SQLITE_DETACH : libc::c_int = 25; /* Database Name   NULL            */
pub const SQLITE_ALTER_TABLE : libc::c_int = 26; /* Database Name   Table Name      */
pub const SQLITE_REINDEX : libc::c_int = 27; /* Index Name      NULL            */
pub const SQLITE_ANALYZE : libc::c_int = 28; /* Table Name      NULL            */
pub const SQLITE_CREATE_VTABLE : libc::c_int = 29; /* Table Name      Module Name     */
pub const SQLITE_DROP_VTABLE : libc::c_int = 30; /* Table Name      Module Name     */
pub const SQLITE_FUNCTION : libc::c_int = 31; /* NULL            Function Name   */
pub const SQLITE_SAVEPOINT : libc::c_int = 32; /* Operation       Savepoint Name  */
pub const SQLITE_COPY : libc::c_int = 0; /* No longer used */
pub const SQLITE_RECURSIVE : libc::c_int = 33; /* NULL            NULL            */
pub const SQLITE_LIMIT_LENGTH : libc::c_int = 0; 
pub const SQLITE_LIMIT_SQL_LENGTH : libc::c_int = 1;
pub const SQLITE_LIMIT_COLUMN : libc::c_int = 2; 
pub const SQLITE_LIMIT_EXPR_DEPTH : libc::c_int = 3;
pub const SQLITE_LIMIT_COMPOUND_SELECT : libc::c_int = 4; 
pub const SQLITE_LIMIT_VDBE_OP : libc::c_int = 5;
pub const SQLITE_LIMIT_FUNCTION_ARG : libc::c_int = 6; 
pub const SQLITE_LIMIT_ATTACHED : libc::c_int = 7;
pub const SQLITE_LIMIT_LIKE_PATTERN_LENGTH : libc::c_int = 8; 
pub const SQLITE_LIMIT_VARIABLE_NUMBER : libc::c_int = 9;
pub const SQLITE_LIMIT_TRIGGER_DEPTH : libc::c_int = 10; 
pub const SQLITE_LIMIT_WORKER_THREADS : libc::c_int = 11;
pub const SQLITE_INTEGER : libc::c_int = 1; 
pub const SQLITE_FLOAT : libc::c_int = 2;
pub const SQLITE_BLOB : libc::c_int = 4; 
pub const SQLITE_NULL : libc::c_int = 5;
pub const SQLITE3_TEXT : libc::c_int = 3; 
pub const SQLITE_TEXT : libc::c_int = SQLITE3_TEXT; 
pub const SQLITE_UTF8 : libc::c_int = 1;    /* IMP: R-37514-35566 */
pub const SQLITE_UTF16LE : libc::c_int = 2; /* IMP: R-03371-37637 */
pub const SQLITE_UTF16BE : libc::c_int = 3; /* IMP: R-51971-34154 */
pub const SQLITE_UTF16 : libc::c_int = 4; /* Use native byte order */
pub const SQLITE_ANY : libc::c_int = 5; /* Deprecated */
pub const SQLITE_UTF16_ALIGNED : libc::c_int = 8; /* sqlite3_create_collation only */
pub const SQLITE_DETERMINISTIC : libc::c_int = 0x800; 
pub const SQLITE_STATIC : libc::c_int = 0;
pub const SQLITE_TRANSIENT : libc::c_int = -1; 
pub const SQLITE_INDEX_CONSTRAINT_EQ : libc::c_int = 2;
pub const SQLITE_INDEX_CONSTRAINT_GT : libc::c_int = 4; 
pub const SQLITE_INDEX_CONSTRAINT_LE : libc::c_int = 8;
pub const SQLITE_INDEX_CONSTRAINT_LT : libc::c_int = 16; 
pub const SQLITE_INDEX_CONSTRAINT_GE : libc::c_int = 32;
pub const SQLITE_INDEX_CONSTRAINT_MATCH : libc::c_int = 64; 
pub const SQLITE_MUTEX_FAST : libc::c_int = 0;
pub const SQLITE_MUTEX_RECURSIVE : libc::c_int = 1; 
pub const SQLITE_MUTEX_STATIC_MASTER : libc::c_int = 2;
pub const SQLITE_MUTEX_STATIC_MEM : libc::c_int = 3; /* sqlite3_malloc() */
pub const SQLITE_MUTEX_STATIC_MEM2 : libc::c_int = 4; /* NOT USED */
pub const SQLITE_MUTEX_STATIC_OPEN : libc::c_int = 4; /* sqlite3BtreeOpen() */
pub const SQLITE_MUTEX_STATIC_PRNG : libc::c_int = 5; /* sqlite3_random() */
pub const SQLITE_MUTEX_STATIC_LRU : libc::c_int = 6; /* lru page list */
pub const SQLITE_MUTEX_STATIC_LRU2 : libc::c_int = 7; /* NOT USED */
pub const SQLITE_MUTEX_STATIC_PMEM : libc::c_int = 7; /* sqlite3PageMalloc() */
pub const SQLITE_MUTEX_STATIC_APP1 : libc::c_int = 8; /* For use by application */
pub const SQLITE_MUTEX_STATIC_APP2 : libc::c_int = 9; /* For use by application */
pub const SQLITE_MUTEX_STATIC_APP3 : libc::c_int = 10; /* For use by application */
pub const SQLITE_TESTCTRL_FIRST : libc::c_int = 5; 
pub const SQLITE_TESTCTRL_PRNG_SAVE : libc::c_int = 5;
pub const SQLITE_TESTCTRL_PRNG_RESTORE : libc::c_int = 6; 
pub const SQLITE_TESTCTRL_PRNG_RESET : libc::c_int = 7;
pub const SQLITE_TESTCTRL_BITVEC_TEST : libc::c_int = 8; 
pub const SQLITE_TESTCTRL_FAULT_INSTALL : libc::c_int = 9;
pub const SQLITE_TESTCTRL_BENIGN_MALLOC_HOOKS : libc::c_int = 10; 
pub const SQLITE_TESTCTRL_PENDING_BYTE : libc::c_int = 11;
pub const SQLITE_TESTCTRL_ASSERT : libc::c_int = 12; 
pub const SQLITE_TESTCTRL_ALWAYS : libc::c_int = 13;
pub const SQLITE_TESTCTRL_RESERVE : libc::c_int = 14; 
pub const SQLITE_TESTCTRL_OPTIMIZATIONS : libc::c_int = 15;
pub const SQLITE_TESTCTRL_ISKEYWORD : libc::c_int = 16; 
pub const SQLITE_TESTCTRL_SCRATCHMALLOC : libc::c_int = 17;
pub const SQLITE_TESTCTRL_LOCALTIME_FAULT : libc::c_int = 18; 
pub const SQLITE_TESTCTRL_EXPLAIN_STMT : libc::c_int = 19;  /* NOT USED */
pub const SQLITE_TESTCTRL_NEVER_CORRUPT : libc::c_int = 20; 
pub const SQLITE_TESTCTRL_VDBE_COVERAGE : libc::c_int = 21;
pub const SQLITE_TESTCTRL_BYTEORDER : libc::c_int = 22; 
pub const SQLITE_TESTCTRL_ISINIT : libc::c_int = 23;
pub const SQLITE_TESTCTRL_SORTER_MMAP : libc::c_int = 24; 
pub const SQLITE_TESTCTRL_IMPOSTER : libc::c_int = 25;
pub const SQLITE_TESTCTRL_LAST : libc::c_int = 25; 
pub const SQLITE_STATUS_MEMORY_USED : libc::c_int = 0;
pub const SQLITE_STATUS_PAGECACHE_USED : libc::c_int = 1; 
pub const SQLITE_STATUS_PAGECACHE_OVERFLOW : libc::c_int = 2;
pub const SQLITE_STATUS_SCRATCH_USED : libc::c_int = 3; 
pub const SQLITE_STATUS_SCRATCH_OVERFLOW : libc::c_int = 4;
pub const SQLITE_STATUS_MALLOC_SIZE : libc::c_int = 5; 
pub const SQLITE_STATUS_PARSER_STACK : libc::c_int = 6;
pub const SQLITE_STATUS_PAGECACHE_SIZE : libc::c_int = 7; 
pub const SQLITE_STATUS_SCRATCH_SIZE : libc::c_int = 8;
pub const SQLITE_STATUS_MALLOC_COUNT : libc::c_int = 9; 
pub const SQLITE_DBSTATUS_LOOKASIDE_USED : libc::c_int = 0;
pub const SQLITE_DBSTATUS_CACHE_USED : libc::c_int = 1; 
pub const SQLITE_DBSTATUS_SCHEMA_USED : libc::c_int = 2;
pub const SQLITE_DBSTATUS_STMT_USED : libc::c_int = 3; 
pub const SQLITE_DBSTATUS_LOOKASIDE_HIT : libc::c_int = 4;
pub const SQLITE_DBSTATUS_LOOKASIDE_MISS_SIZE : libc::c_int = 5; 
pub const SQLITE_DBSTATUS_LOOKASIDE_MISS_FULL : libc::c_int = 6;
pub const SQLITE_DBSTATUS_CACHE_HIT : libc::c_int = 7; 
pub const SQLITE_DBSTATUS_CACHE_MISS : libc::c_int = 8;
pub const SQLITE_DBSTATUS_CACHE_WRITE : libc::c_int = 9; 
pub const SQLITE_DBSTATUS_DEFERRED_FKS : libc::c_int = 10;
pub const SQLITE_DBSTATUS_MAX : libc::c_int = 10; /* Largest defined DBSTATUS */
pub const SQLITE_STMTSTATUS_FULLSCAN_STEP : libc::c_int = 1; 
pub const SQLITE_STMTSTATUS_SORT : libc::c_int = 2;
pub const SQLITE_STMTSTATUS_AUTOINDEX : libc::c_int = 3; 
pub const SQLITE_STMTSTATUS_VM_STEP : libc::c_int = 4;
pub const SQLITE_CHECKPOINT_PASSIVE : libc::c_int = 0; /* Do as much as possible w/o blocking */
pub const SQLITE_CHECKPOINT_FULL : libc::c_int = 1; /* Wait for writers, then checkpoint */
pub const SQLITE_CHECKPOINT_RESTART : libc::c_int = 2; /* Like FULL but wait for for readers */
pub const SQLITE_CHECKPOINT_TRUNCATE : libc::c_int = 3; /* Like RESTART but also truncate WAL */
pub const SQLITE_VTAB_CONSTRAINT_SUPPORT : libc::c_int = 1; 
pub const SQLITE_ROLLBACK : libc::c_int = 1;
/* pub const SQLITE_IGNORE : libc::c_int = 2; // Also used by sqlite3_authorizer() callback */
pub const SQLITE_FAIL : libc::c_int = 3; /* 
pub const SQLITE_ABORT : libc::c_int = 4;  // Also an error code */
pub const SQLITE_REPLACE : libc::c_int = 5; 
pub const SQLITE_SCANSTAT_NLOOP : libc::c_int = 0;
pub const SQLITE_SCANSTAT_NVISIT : libc::c_int = 1; 
pub const SQLITE_SCANSTAT_EST : libc::c_int = 2;
pub const SQLITE_SCANSTAT_NAME : libc::c_int = 3; 
pub const SQLITE_SCANSTAT_EXPLAIN : libc::c_int = 4;
pub const SQLITE_SCANSTAT_SELECTID : libc::c_int = 5; 
pub const NOT_WITHIN : libc::c_int = 0; /* Object completely outside of query region */
pub const PARTLY_WITHIN : libc::c_int = 1; /* Object partially overlaps query region */
pub const FULLY_WITHIN : libc::c_int = 2; /* Object fully contained within query region */

/* automatically generated by rust-bindgen */

//pub type va_list = __builtin_va_list;
//pub type __gnuc_va_list = __builtin_va_list;
pub enum Struct_sqlite3 { }
pub type sqlite3 = Struct_sqlite3;
pub type sqlite_int64 = libc::c_longlong;
pub type sqlite_uint64 = libc::c_ulonglong;
pub type sqlite3_int64 = sqlite_int64;
pub type sqlite3_uint64 = sqlite_uint64;
pub type sqlite3_callback =
    ::std::option::Option<extern "C" fn(arg1: *mut libc::c_void,
                                        arg2: libc::c_int,
                                        arg3: *mut *mut libc::c_char,
                                        arg4: *mut *mut libc::c_char)
                              -> libc::c_int>;
pub type sqlite3_file = Struct_sqlite3_file;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_sqlite3_file {
    pub pMethods: *const Struct_sqlite3_io_methods,
}
impl ::std::clone::Clone for Struct_sqlite3_file {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_sqlite3_file {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type sqlite3_io_methods = Struct_sqlite3_io_methods;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_sqlite3_io_methods {
    pub iVersion: libc::c_int,
    pub xClose: ::std::option::Option<extern "C" fn(arg1: *mut sqlite3_file)
                                          -> libc::c_int>,
    pub xRead: ::std::option::Option<extern "C" fn(arg1: *mut sqlite3_file,
                                                   arg2: *mut libc::c_void,
                                                   iAmt: libc::c_int,
                                                   iOfst: sqlite3_int64)
                                         -> libc::c_int>,
    pub xWrite: ::std::option::Option<extern "C" fn(arg1: *mut sqlite3_file,
                                                    arg2:
                                                        *const libc::c_void,
                                                    iAmt: libc::c_int,
                                                    iOfst: sqlite3_int64)
                                          -> libc::c_int>,
    pub xTruncate: ::std::option::Option<extern "C" fn(arg1:
                                                           *mut sqlite3_file,
                                                       size: sqlite3_int64)
                                             -> libc::c_int>,
    pub xSync: ::std::option::Option<extern "C" fn(arg1: *mut sqlite3_file,
                                                   flags: libc::c_int)
                                         -> libc::c_int>,
    pub xFileSize: ::std::option::Option<extern "C" fn(arg1:
                                                           *mut sqlite3_file,
                                                       pSize:
                                                           *mut sqlite3_int64)
                                             -> libc::c_int>,
    pub xLock: ::std::option::Option<extern "C" fn(arg1: *mut sqlite3_file,
                                                   arg2: libc::c_int)
                                         -> libc::c_int>,
    pub xUnlock: ::std::option::Option<extern "C" fn(arg1: *mut sqlite3_file,
                                                     arg2: libc::c_int)
                                           -> libc::c_int>,
    pub xCheckReservedLock: ::std::option::Option<extern "C" fn(arg1:
                                                                    *mut sqlite3_file,
                                                                pResOut:
                                                                    *mut libc::c_int)
                                                      -> libc::c_int>,
    pub xFileControl: ::std::option::Option<extern "C" fn(arg1:
                                                              *mut sqlite3_file,
                                                          op: libc::c_int,
                                                          pArg:
                                                              *mut libc::c_void)
                                                -> libc::c_int>,
    pub xSectorSize: ::std::option::Option<extern "C" fn(arg1:
                                                             *mut sqlite3_file)
                                               -> libc::c_int>,
    pub xDeviceCharacteristics: ::std::option::Option<extern "C" fn(arg1:
                                                                        *mut sqlite3_file)
                                                          -> libc::c_int>,
    pub xShmMap: ::std::option::Option<extern "C" fn(arg1: *mut sqlite3_file,
                                                     iPg: libc::c_int,
                                                     pgsz: libc::c_int,
                                                     arg2: libc::c_int,
                                                     arg3:
                                                         *mut *mut libc::c_void)
                                           -> libc::c_int>,
    pub xShmLock: ::std::option::Option<extern "C" fn(arg1: *mut sqlite3_file,
                                                      offset: libc::c_int,
                                                      n: libc::c_int,
                                                      flags: libc::c_int)
                                            -> libc::c_int>,
    pub xShmBarrier: ::std::option::Option<extern "C" fn(arg1:
                                                             *mut sqlite3_file)
                                               -> ()>,
    pub xShmUnmap: ::std::option::Option<extern "C" fn(arg1:
                                                           *mut sqlite3_file,
                                                       deleteFlag:
                                                           libc::c_int)
                                             -> libc::c_int>,
    pub xFetch: ::std::option::Option<extern "C" fn(arg1: *mut sqlite3_file,
                                                    iOfst: sqlite3_int64,
                                                    iAmt: libc::c_int,
                                                    pp:
                                                        *mut *mut libc::c_void)
                                          -> libc::c_int>,
    pub xUnfetch: ::std::option::Option<extern "C" fn(arg1: *mut sqlite3_file,
                                                      iOfst: sqlite3_int64,
                                                      p: *mut libc::c_void)
                                            -> libc::c_int>,
}
impl ::std::clone::Clone for Struct_sqlite3_io_methods {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_sqlite3_io_methods {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub enum Struct_sqlite3_mutex { }
pub type sqlite3_mutex = Struct_sqlite3_mutex;
pub type sqlite3_vfs = Struct_sqlite3_vfs;
pub type sqlite3_syscall_ptr = ::std::option::Option<extern "C" fn() -> ()>;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_sqlite3_vfs {
    pub iVersion: libc::c_int,
    pub szOsFile: libc::c_int,
    pub mxPathname: libc::c_int,
    pub pNext: *mut sqlite3_vfs,
    pub zName: *const libc::c_char,
    pub pAppData: *mut libc::c_void,
    pub xOpen: ::std::option::Option<extern "C" fn(arg1: *mut sqlite3_vfs,
                                                   zName:
                                                       *const libc::c_char,
                                                   arg2: *mut sqlite3_file,
                                                   flags: libc::c_int,
                                                   pOutFlags:
                                                       *mut libc::c_int)
                                         -> libc::c_int>,
    pub xDelete: ::std::option::Option<extern "C" fn(arg1: *mut sqlite3_vfs,
                                                     zName:
                                                         *const libc::c_char,
                                                     syncDir: libc::c_int)
                                           -> libc::c_int>,
    pub xAccess: ::std::option::Option<extern "C" fn(arg1: *mut sqlite3_vfs,
                                                     zName:
                                                         *const libc::c_char,
                                                     flags: libc::c_int,
                                                     pResOut:
                                                         *mut libc::c_int)
                                           -> libc::c_int>,
    pub xFullPathname: ::std::option::Option<extern "C" fn(arg1:
                                                               *mut sqlite3_vfs,
                                                           zName:
                                                               *const libc::c_char,
                                                           nOut:
                                                               libc::c_int,
                                                           zOut:
                                                               *mut libc::c_char)
                                                 -> libc::c_int>,
    pub xDlOpen: ::std::option::Option<extern "C" fn(arg1: *mut sqlite3_vfs,
                                                     zFilename:
                                                         *const libc::c_char)
                                           -> *mut libc::c_void>,
    pub xDlError: ::std::option::Option<extern "C" fn(arg1: *mut sqlite3_vfs,
                                                      nByte: libc::c_int,
                                                      zErrMsg:
                                                          *mut libc::c_char)
                                            -> ()>,
    pub xDlSym: ::std::option::Option<extern "C" fn(arg1: *mut sqlite3_vfs,
                                                    arg2: *mut libc::c_void,
                                                    zSymbol:
                                                        *const libc::c_char)
                                          ->
                                              ::std::option::Option<extern "C" fn(arg1:
                                                                                      *mut sqlite3_vfs,
                                                                                  arg2:
                                                                                      *mut libc::c_void,
                                                                                  zSymbol:
                                                                                      *const libc::c_char)
                                                                        ->
                                                                            ()>>,
    pub xDlClose: ::std::option::Option<extern "C" fn(arg1: *mut sqlite3_vfs,
                                                      arg2:
                                                          *mut libc::c_void)
                                            -> ()>,
    pub xRandomness: ::std::option::Option<extern "C" fn(arg1:
                                                             *mut sqlite3_vfs,
                                                         nByte: libc::c_int,
                                                         zOut:
                                                             *mut libc::c_char)
                                               -> libc::c_int>,
    pub xSleep: ::std::option::Option<extern "C" fn(arg1: *mut sqlite3_vfs,
                                                    microseconds:
                                                        libc::c_int)
                                          -> libc::c_int>,
    pub xCurrentTime: ::std::option::Option<extern "C" fn(arg1:
                                                              *mut sqlite3_vfs,
                                                          arg2:
                                                              *mut libc::c_double)
                                                -> libc::c_int>,
    pub xGetLastError: ::std::option::Option<extern "C" fn(arg1:
                                                               *mut sqlite3_vfs,
                                                           arg2:
                                                               libc::c_int,
                                                           arg3:
                                                               *mut libc::c_char)
                                                 -> libc::c_int>,
    pub xCurrentTimeInt64: ::std::option::Option<extern "C" fn(arg1:
                                                                   *mut sqlite3_vfs,
                                                               arg2:
                                                                   *mut sqlite3_int64)
                                                     -> libc::c_int>,
    pub xSetSystemCall: ::std::option::Option<extern "C" fn(arg1:
                                                                *mut sqlite3_vfs,
                                                            zName:
                                                                *const libc::c_char,
                                                            arg2:
                                                                sqlite3_syscall_ptr)
                                                  -> libc::c_int>,
    pub xGetSystemCall: ::std::option::Option<extern "C" fn(arg1:
                                                                *mut sqlite3_vfs,
                                                            zName:
                                                                *const libc::c_char)
                                                  -> sqlite3_syscall_ptr>,
    pub xNextSystemCall: ::std::option::Option<extern "C" fn(arg1:
                                                                 *mut sqlite3_vfs,
                                                             zName:
                                                                 *const libc::c_char)
                                                   -> *const libc::c_char>,
}
impl ::std::clone::Clone for Struct_sqlite3_vfs {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_sqlite3_vfs {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type sqlite3_mem_methods = Struct_sqlite3_mem_methods;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_sqlite3_mem_methods {
    pub xMalloc: ::std::option::Option<extern "C" fn(arg1: libc::c_int)
                                           -> *mut libc::c_void>,
    pub xFree: ::std::option::Option<extern "C" fn(arg1: *mut libc::c_void)
                                         -> ()>,
    pub xRealloc: ::std::option::Option<extern "C" fn(arg1:
                                                          *mut libc::c_void,
                                                      arg2: libc::c_int)
                                            -> *mut libc::c_void>,
    pub xSize: ::std::option::Option<extern "C" fn(arg1: *mut libc::c_void)
                                         -> libc::c_int>,
    pub xRoundup: ::std::option::Option<extern "C" fn(arg1: libc::c_int)
                                            -> libc::c_int>,
    pub xInit: ::std::option::Option<extern "C" fn(arg1: *mut libc::c_void)
                                         -> libc::c_int>,
    pub xShutdown: ::std::option::Option<extern "C" fn(arg1:
                                                           *mut libc::c_void)
                                             -> ()>,
    pub pAppData: *mut libc::c_void,
}
impl ::std::clone::Clone for Struct_sqlite3_mem_methods {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_sqlite3_mem_methods {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub enum Struct_sqlite3_stmt { }
pub type sqlite3_stmt = Struct_sqlite3_stmt;
pub enum Struct_Mem { }
pub type sqlite3_value = Struct_Mem;
pub enum Struct_sqlite3_context { }
pub type sqlite3_context = Struct_sqlite3_context;
pub type sqlite3_destructor_type =
    ::std::option::Option<extern "C" fn(arg1: *mut libc::c_void) -> ()>;
pub type sqlite3_vtab = Struct_sqlite3_vtab;
pub type sqlite3_index_info = Struct_sqlite3_index_info;
pub type sqlite3_vtab_cursor = Struct_sqlite3_vtab_cursor;
pub type sqlite3_module = Struct_sqlite3_module;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_sqlite3_module {
    pub iVersion: libc::c_int,
    pub xCreate: ::std::option::Option<extern "C" fn(arg1: *mut sqlite3,
                                                     pAux:
                                                         *mut libc::c_void,
                                                     argc: libc::c_int,
                                                     argv:
                                                         *const *const libc::c_char,
                                                     ppVTab:
                                                         *mut *mut sqlite3_vtab,
                                                     arg2:
                                                         *mut *mut libc::c_char)
                                           -> libc::c_int>,
    pub xConnect: ::std::option::Option<extern "C" fn(arg1: *mut sqlite3,
                                                      pAux:
                                                          *mut libc::c_void,
                                                      argc: libc::c_int,
                                                      argv:
                                                          *const *const libc::c_char,
                                                      ppVTab:
                                                          *mut *mut sqlite3_vtab,
                                                      arg2:
                                                          *mut *mut libc::c_char)
                                            -> libc::c_int>,
    pub xBestIndex: ::std::option::Option<extern "C" fn(pVTab:
                                                            *mut sqlite3_vtab,
                                                        arg1:
                                                            *mut sqlite3_index_info)
                                              -> libc::c_int>,
    pub xDisconnect: ::std::option::Option<extern "C" fn(pVTab:
                                                             *mut sqlite3_vtab)
                                               -> libc::c_int>,
    pub xDestroy: ::std::option::Option<extern "C" fn(pVTab:
                                                          *mut sqlite3_vtab)
                                            -> libc::c_int>,
    pub xOpen: ::std::option::Option<extern "C" fn(pVTab: *mut sqlite3_vtab,
                                                   ppCursor:
                                                       *mut *mut sqlite3_vtab_cursor)
                                         -> libc::c_int>,
    pub xClose: ::std::option::Option<extern "C" fn(arg1:
                                                        *mut sqlite3_vtab_cursor)
                                          -> libc::c_int>,
    pub xFilter: ::std::option::Option<extern "C" fn(arg1:
                                                         *mut sqlite3_vtab_cursor,
                                                     idxNum: libc::c_int,
                                                     idxStr:
                                                         *const libc::c_char,
                                                     argc: libc::c_int,
                                                     argv:
                                                         *mut *mut sqlite3_value)
                                           -> libc::c_int>,
    pub xNext: ::std::option::Option<extern "C" fn(arg1:
                                                       *mut sqlite3_vtab_cursor)
                                         -> libc::c_int>,
    pub xEof: ::std::option::Option<extern "C" fn(arg1:
                                                      *mut sqlite3_vtab_cursor)
                                        -> libc::c_int>,
    pub xColumn: ::std::option::Option<extern "C" fn(arg1:
                                                         *mut sqlite3_vtab_cursor,
                                                     arg2:
                                                         *mut sqlite3_context,
                                                     arg3: libc::c_int)
                                           -> libc::c_int>,
    pub xRowid: ::std::option::Option<extern "C" fn(arg1:
                                                        *mut sqlite3_vtab_cursor,
                                                    pRowid:
                                                        *mut sqlite3_int64)
                                          -> libc::c_int>,
    pub xUpdate: ::std::option::Option<extern "C" fn(arg1: *mut sqlite3_vtab,
                                                     arg2: libc::c_int,
                                                     arg3:
                                                         *mut *mut sqlite3_value,
                                                     arg4: *mut sqlite3_int64)
                                           -> libc::c_int>,
    pub xBegin: ::std::option::Option<extern "C" fn(pVTab: *mut sqlite3_vtab)
                                          -> libc::c_int>,
    pub xSync: ::std::option::Option<extern "C" fn(pVTab: *mut sqlite3_vtab)
                                         -> libc::c_int>,
    pub xCommit: ::std::option::Option<extern "C" fn(pVTab: *mut sqlite3_vtab)
                                           -> libc::c_int>,
    pub xRollback: ::std::option::Option<extern "C" fn(pVTab:
                                                           *mut sqlite3_vtab)
                                             -> libc::c_int>,
    pub xFindFunction: ::std::option::Option<extern "C" fn(pVtab:
                                                               *mut sqlite3_vtab,
                                                           nArg:
                                                               libc::c_int,
                                                           zName:
                                                               *const libc::c_char,
                                                           pxFunc:
                                                               *mut ::std::option::Option<extern "C" fn(arg1:
                                                                                                            *mut sqlite3_context,
                                                                                                        arg2:
                                                                                                            libc::c_int,
                                                                                                        arg3:
                                                                                                            *mut *mut sqlite3_value)
                                                                                              ->
                                                                                                  ()>,
                                                           ppArg:
                                                               *mut *mut libc::c_void)
                                                 -> libc::c_int>,
    pub xRename: ::std::option::Option<extern "C" fn(pVtab: *mut sqlite3_vtab,
                                                     zNew:
                                                         *const libc::c_char)
                                           -> libc::c_int>,
    pub xSavepoint: ::std::option::Option<extern "C" fn(pVTab:
                                                            *mut sqlite3_vtab,
                                                        arg1: libc::c_int)
                                              -> libc::c_int>,
    pub xRelease: ::std::option::Option<extern "C" fn(pVTab:
                                                          *mut sqlite3_vtab,
                                                      arg1: libc::c_int)
                                            -> libc::c_int>,
    pub xRollbackTo: ::std::option::Option<extern "C" fn(pVTab:
                                                             *mut sqlite3_vtab,
                                                         arg1: libc::c_int)
                                               -> libc::c_int>,
}
impl ::std::clone::Clone for Struct_sqlite3_module {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_sqlite3_module {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_sqlite3_index_info {
    pub nConstraint: libc::c_int,
    pub aConstraint: *mut Struct_sqlite3_index_constraint,
    pub nOrderBy: libc::c_int,
    pub aOrderBy: *mut Struct_sqlite3_index_orderby,
    pub aConstraintUsage: *mut Struct_sqlite3_index_constraint_usage,
    pub idxNum: libc::c_int,
    pub idxStr: *mut libc::c_char,
    pub needToFreeIdxStr: libc::c_int,
    pub orderByConsumed: libc::c_int,
    pub estimatedCost: libc::c_double,
    pub estimatedRows: sqlite3_int64,
}
impl ::std::clone::Clone for Struct_sqlite3_index_info {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_sqlite3_index_info {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_sqlite3_index_constraint {
    pub iColumn: libc::c_int,
    pub op: libc::c_uchar,
    pub usable: libc::c_uchar,
    pub iTermOffset: libc::c_int,
}
impl ::std::clone::Clone for Struct_sqlite3_index_constraint {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_sqlite3_index_constraint {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_sqlite3_index_orderby {
    pub iColumn: libc::c_int,
    pub desc: libc::c_uchar,
}
impl ::std::clone::Clone for Struct_sqlite3_index_orderby {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_sqlite3_index_orderby {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_sqlite3_index_constraint_usage {
    pub argvIndex: libc::c_int,
    pub omit: libc::c_uchar,
}
impl ::std::clone::Clone for Struct_sqlite3_index_constraint_usage {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_sqlite3_index_constraint_usage {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_sqlite3_vtab {
    pub pModule: *const sqlite3_module,
    pub nRef: libc::c_int,
    pub zErrMsg: *mut libc::c_char,
}
impl ::std::clone::Clone for Struct_sqlite3_vtab {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_sqlite3_vtab {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_sqlite3_vtab_cursor {
    pub pVtab: *mut sqlite3_vtab,
}
impl ::std::clone::Clone for Struct_sqlite3_vtab_cursor {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_sqlite3_vtab_cursor {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub enum Struct_sqlite3_blob { }
pub type sqlite3_blob = Struct_sqlite3_blob;
pub type sqlite3_mutex_methods = Struct_sqlite3_mutex_methods;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_sqlite3_mutex_methods {
    pub xMutexInit: ::std::option::Option<extern "C" fn() -> libc::c_int>,
    pub xMutexEnd: ::std::option::Option<extern "C" fn() -> libc::c_int>,
    pub xMutexAlloc: ::std::option::Option<extern "C" fn(arg1: libc::c_int)
                                               -> *mut sqlite3_mutex>,
    pub xMutexFree: ::std::option::Option<extern "C" fn(arg1:
                                                            *mut sqlite3_mutex)
                                              -> ()>,
    pub xMutexEnter: ::std::option::Option<extern "C" fn(arg1:
                                                             *mut sqlite3_mutex)
                                               -> ()>,
    pub xMutexTry: ::std::option::Option<extern "C" fn(arg1:
                                                           *mut sqlite3_mutex)
                                             -> libc::c_int>,
    pub xMutexLeave: ::std::option::Option<extern "C" fn(arg1:
                                                             *mut sqlite3_mutex)
                                               -> ()>,
    pub xMutexHeld: ::std::option::Option<extern "C" fn(arg1:
                                                            *mut sqlite3_mutex)
                                              -> libc::c_int>,
    pub xMutexNotheld: ::std::option::Option<extern "C" fn(arg1:
                                                               *mut sqlite3_mutex)
                                                 -> libc::c_int>,
}
impl ::std::clone::Clone for Struct_sqlite3_mutex_methods {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_sqlite3_mutex_methods {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub enum Struct_sqlite3_pcache { }
pub type sqlite3_pcache = Struct_sqlite3_pcache;
pub type sqlite3_pcache_page = Struct_sqlite3_pcache_page;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_sqlite3_pcache_page {
    pub pBuf: *mut libc::c_void,
    pub pExtra: *mut libc::c_void,
}
impl ::std::clone::Clone for Struct_sqlite3_pcache_page {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_sqlite3_pcache_page {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type sqlite3_pcache_methods2 = Struct_sqlite3_pcache_methods2;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_sqlite3_pcache_methods2 {
    pub iVersion: libc::c_int,
    pub pArg: *mut libc::c_void,
    pub xInit: ::std::option::Option<extern "C" fn(arg1: *mut libc::c_void)
                                         -> libc::c_int>,
    pub xShutdown: ::std::option::Option<extern "C" fn(arg1:
                                                           *mut libc::c_void)
                                             -> ()>,
    pub xCreate: ::std::option::Option<extern "C" fn(szPage: libc::c_int,
                                                     szExtra: libc::c_int,
                                                     bPurgeable:
                                                         libc::c_int)
                                           -> *mut sqlite3_pcache>,
    pub xCachesize: ::std::option::Option<extern "C" fn(arg1:
                                                            *mut sqlite3_pcache,
                                                        nCachesize:
                                                            libc::c_int)
                                              -> ()>,
    pub xPagecount: ::std::option::Option<extern "C" fn(arg1:
                                                            *mut sqlite3_pcache)
                                              -> libc::c_int>,
    pub xFetch: ::std::option::Option<extern "C" fn(arg1: *mut sqlite3_pcache,
                                                    key: libc::c_uint,
                                                    createFlag: libc::c_int)
                                          -> *mut sqlite3_pcache_page>,
    pub xUnpin: ::std::option::Option<extern "C" fn(arg1: *mut sqlite3_pcache,
                                                    arg2:
                                                        *mut sqlite3_pcache_page,
                                                    discard: libc::c_int)
                                          -> ()>,
    pub xRekey: ::std::option::Option<extern "C" fn(arg1: *mut sqlite3_pcache,
                                                    arg2:
                                                        *mut sqlite3_pcache_page,
                                                    oldKey: libc::c_uint,
                                                    newKey: libc::c_uint)
                                          -> ()>,
    pub xTruncate: ::std::option::Option<extern "C" fn(arg1:
                                                           *mut sqlite3_pcache,
                                                       iLimit: libc::c_uint)
                                             -> ()>,
    pub xDestroy: ::std::option::Option<extern "C" fn(arg1:
                                                          *mut sqlite3_pcache)
                                            -> ()>,
    pub xShrink: ::std::option::Option<extern "C" fn(arg1:
                                                         *mut sqlite3_pcache)
                                           -> ()>,
}
impl ::std::clone::Clone for Struct_sqlite3_pcache_methods2 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_sqlite3_pcache_methods2 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type sqlite3_pcache_methods = Struct_sqlite3_pcache_methods;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_sqlite3_pcache_methods {
    pub pArg: *mut libc::c_void,
    pub xInit: ::std::option::Option<extern "C" fn(arg1: *mut libc::c_void)
                                         -> libc::c_int>,
    pub xShutdown: ::std::option::Option<extern "C" fn(arg1:
                                                           *mut libc::c_void)
                                             -> ()>,
    pub xCreate: ::std::option::Option<extern "C" fn(szPage: libc::c_int,
                                                     bPurgeable:
                                                         libc::c_int)
                                           -> *mut sqlite3_pcache>,
    pub xCachesize: ::std::option::Option<extern "C" fn(arg1:
                                                            *mut sqlite3_pcache,
                                                        nCachesize:
                                                            libc::c_int)
                                              -> ()>,
    pub xPagecount: ::std::option::Option<extern "C" fn(arg1:
                                                            *mut sqlite3_pcache)
                                              -> libc::c_int>,
    pub xFetch: ::std::option::Option<extern "C" fn(arg1: *mut sqlite3_pcache,
                                                    key: libc::c_uint,
                                                    createFlag: libc::c_int)
                                          -> *mut libc::c_void>,
    pub xUnpin: ::std::option::Option<extern "C" fn(arg1: *mut sqlite3_pcache,
                                                    arg2: *mut libc::c_void,
                                                    discard: libc::c_int)
                                          -> ()>,
    pub xRekey: ::std::option::Option<extern "C" fn(arg1: *mut sqlite3_pcache,
                                                    arg2: *mut libc::c_void,
                                                    oldKey: libc::c_uint,
                                                    newKey: libc::c_uint)
                                          -> ()>,
    pub xTruncate: ::std::option::Option<extern "C" fn(arg1:
                                                           *mut sqlite3_pcache,
                                                       iLimit: libc::c_uint)
                                             -> ()>,
    pub xDestroy: ::std::option::Option<extern "C" fn(arg1:
                                                          *mut sqlite3_pcache)
                                            -> ()>,
}
impl ::std::clone::Clone for Struct_sqlite3_pcache_methods {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_sqlite3_pcache_methods {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub enum Struct_sqlite3_backup { }
pub type sqlite3_backup = Struct_sqlite3_backup;
pub type sqlite3_rtree_geometry = Struct_sqlite3_rtree_geometry;
pub type sqlite3_rtree_query_info = Struct_sqlite3_rtree_query_info;
pub type sqlite3_rtree_dbl = libc::c_double;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_sqlite3_rtree_geometry {
    pub pContext: *mut libc::c_void,
    pub nParam: libc::c_int,
    pub aParam: *mut sqlite3_rtree_dbl,
    pub pUser: *mut libc::c_void,
    pub xDelUser: ::std::option::Option<extern "C" fn(arg1:
                                                          *mut libc::c_void)
                                            -> ()>,
}
impl ::std::clone::Clone for Struct_sqlite3_rtree_geometry {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_sqlite3_rtree_geometry {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_sqlite3_rtree_query_info {
    pub pContext: *mut libc::c_void,
    pub nParam: libc::c_int,
    pub aParam: *mut sqlite3_rtree_dbl,
    pub pUser: *mut libc::c_void,
    pub xDelUser: ::std::option::Option<extern "C" fn(arg1:
                                                          *mut libc::c_void)
                                            -> ()>,
    pub aCoord: *mut sqlite3_rtree_dbl,
    pub anQueue: *mut libc::c_uint,
    pub nCoord: libc::c_int,
    pub iLevel: libc::c_int,
    pub mxLevel: libc::c_int,
    pub iRowid: sqlite3_int64,
    pub rParentScore: sqlite3_rtree_dbl,
    pub eParentWithin: libc::c_int,
    pub eWithin: libc::c_int,
    pub rScore: sqlite3_rtree_dbl,
}
impl ::std::clone::Clone for Struct_sqlite3_rtree_query_info {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_sqlite3_rtree_query_info {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
extern "C" {
    pub static mut sqlite3_version: *const libc::c_char;
    pub static mut sqlite3_temp_directory: *mut libc::c_char;
    pub static mut sqlite3_data_directory: *mut libc::c_char;
}
extern "C" {
    pub fn sqlite3_libversion() -> *const libc::c_char;
    pub fn sqlite3_sourceid() -> *const libc::c_char;
    pub fn sqlite3_libversion_number() -> libc::c_int;
    pub fn sqlite3_compileoption_used(zOptName: *const libc::c_char)
     -> libc::c_int;
    pub fn sqlite3_compileoption_get(N: libc::c_int)
     -> *const libc::c_char;
    pub fn sqlite3_threadsafe() -> libc::c_int;
    pub fn sqlite3_close(arg1: *mut sqlite3) -> libc::c_int;
    pub fn sqlite3_close_v2(arg1: *mut sqlite3) -> libc::c_int;
    pub fn sqlite3_exec(arg1: *mut sqlite3, sql: *const libc::c_char,
                        callback:
                            ::std::option::Option<extern "C" fn(arg1:
                                                                    *mut libc::c_void,
                                                                arg2:
                                                                    libc::c_int,
                                                                arg3:
                                                                    *mut *mut libc::c_char,
                                                                arg4:
                                                                    *mut *mut libc::c_char)
                                                      -> libc::c_int>,
                        arg2: *mut libc::c_void,
                        errmsg: *mut *mut libc::c_char) -> libc::c_int;
    pub fn sqlite3_initialize() -> libc::c_int;
    pub fn sqlite3_shutdown() -> libc::c_int;
    pub fn sqlite3_os_init() -> libc::c_int;
    pub fn sqlite3_os_end() -> libc::c_int;
    pub fn sqlite3_config(arg1: libc::c_int, ...) -> libc::c_int;
    pub fn sqlite3_db_config(arg1: *mut sqlite3, op: libc::c_int, ...)
     -> libc::c_int;
    pub fn sqlite3_extended_result_codes(arg1: *mut sqlite3,
                                         onoff: libc::c_int)
     -> libc::c_int;
    pub fn sqlite3_last_insert_rowid(arg1: *mut sqlite3) -> sqlite3_int64;
    pub fn sqlite3_changes(arg1: *mut sqlite3) -> libc::c_int;
    pub fn sqlite3_total_changes(arg1: *mut sqlite3) -> libc::c_int;
    pub fn sqlite3_interrupt(arg1: *mut sqlite3) -> ();
    pub fn sqlite3_complete(sql: *const libc::c_char) -> libc::c_int;
    pub fn sqlite3_complete16(sql: *const libc::c_void) -> libc::c_int;
    pub fn sqlite3_busy_handler(arg1: *mut sqlite3,
                                arg2:
                                    ::std::option::Option<extern "C" fn(arg1:
                                                                            *mut libc::c_void,
                                                                        arg2:
                                                                            libc::c_int)
                                                              ->
                                                                  libc::c_int>,
                                arg3: *mut libc::c_void) -> libc::c_int;
    pub fn sqlite3_busy_timeout(arg1: *mut sqlite3, ms: libc::c_int)
     -> libc::c_int;
    pub fn sqlite3_get_table(db: *mut sqlite3, zSql: *const libc::c_char,
                             pazResult: *mut *mut *mut libc::c_char,
                             pnRow: *mut libc::c_int,
                             pnColumn: *mut libc::c_int,
                             pzErrmsg: *mut *mut libc::c_char)
     -> libc::c_int;
    pub fn sqlite3_free_table(result: *mut *mut libc::c_char) -> ();
    pub fn sqlite3_mprintf(arg1: *const libc::c_char, ...)
     -> *mut libc::c_char;
    //pub fn sqlite3_vmprintf(arg1: *const libc::c_char, arg2: va_list)
    // -> *mut libc::c_char;
    pub fn sqlite3_snprintf(arg1: libc::c_int, arg2: *mut libc::c_char,
                            arg3: *const libc::c_char, ...)
     -> *mut libc::c_char;
    //pub fn sqlite3_vsnprintf(arg1: libc::c_int, arg2: *mut libc::c_char,
    //                         arg3: *const libc::c_char, arg4: va_list)
    // -> *mut libc::c_char;
    pub fn sqlite3_malloc(arg1: libc::c_int) -> *mut libc::c_void;
    pub fn sqlite3_malloc64(arg1: sqlite3_uint64) -> *mut libc::c_void;
    pub fn sqlite3_realloc(arg1: *mut libc::c_void, arg2: libc::c_int)
     -> *mut libc::c_void;
    pub fn sqlite3_realloc64(arg1: *mut libc::c_void, arg2: sqlite3_uint64)
     -> *mut libc::c_void;
    pub fn sqlite3_free(arg1: *mut libc::c_void) -> ();
    pub fn sqlite3_msize(arg1: *mut libc::c_void) -> sqlite3_uint64;
    pub fn sqlite3_memory_used() -> sqlite3_int64;
    pub fn sqlite3_memory_highwater(resetFlag: libc::c_int)
     -> sqlite3_int64;
    pub fn sqlite3_randomness(N: libc::c_int, P: *mut libc::c_void) -> ();
    pub fn sqlite3_set_authorizer(arg1: *mut sqlite3,
                                  xAuth:
                                      ::std::option::Option<extern "C" fn(arg1:
                                                                              *mut libc::c_void,
                                                                          arg2:
                                                                              libc::c_int,
                                                                          arg3:
                                                                              *const libc::c_char,
                                                                          arg4:
                                                                              *const libc::c_char,
                                                                          arg5:
                                                                              *const libc::c_char,
                                                                          arg6:
                                                                              *const libc::c_char)
                                                                ->
                                                                    libc::c_int>,
                                  pUserData: *mut libc::c_void)
     -> libc::c_int;
    pub fn sqlite3_trace(arg1: *mut sqlite3,
                         xTrace:
                             ::std::option::Option<extern "C" fn(arg1:
                                                                     *mut libc::c_void,
                                                                 arg2:
                                                                     *const libc::c_char)
                                                       -> ()>,
                         arg2: *mut libc::c_void) -> *mut libc::c_void;
    pub fn sqlite3_profile(arg1: *mut sqlite3,
                           xProfile:
                               ::std::option::Option<extern "C" fn(arg1:
                                                                       *mut libc::c_void,
                                                                   arg2:
                                                                       *const libc::c_char,
                                                                   arg3:
                                                                       sqlite3_uint64)
                                                         -> ()>,
                           arg2: *mut libc::c_void) -> *mut libc::c_void;
    pub fn sqlite3_progress_handler(arg1: *mut sqlite3, arg2: libc::c_int,
                                    arg3:
                                        ::std::option::Option<extern "C" fn(arg1:
                                                                                *mut libc::c_void)
                                                                  ->
                                                                      libc::c_int>,
                                    arg4: *mut libc::c_void) -> ();
    pub fn sqlite3_open(filename: *const libc::c_char,
                        ppDb: *mut *mut sqlite3) -> libc::c_int;
    pub fn sqlite3_open16(filename: *const libc::c_void,
                          ppDb: *mut *mut sqlite3) -> libc::c_int;
    pub fn sqlite3_open_v2(filename: *const libc::c_char,
                           ppDb: *mut *mut sqlite3, flags: libc::c_int,
                           zVfs: *const libc::c_char) -> libc::c_int;
    pub fn sqlite3_uri_parameter(zFilename: *const libc::c_char,
                                 zParam: *const libc::c_char)
     -> *const libc::c_char;
    pub fn sqlite3_uri_boolean(zFile: *const libc::c_char,
                               zParam: *const libc::c_char,
                               bDefault: libc::c_int) -> libc::c_int;
    pub fn sqlite3_uri_int64(arg1: *const libc::c_char,
                             arg2: *const libc::c_char, arg3: sqlite3_int64)
     -> sqlite3_int64;
    pub fn sqlite3_errcode(db: *mut sqlite3) -> libc::c_int;
    pub fn sqlite3_extended_errcode(db: *mut sqlite3) -> libc::c_int;
    pub fn sqlite3_errmsg(arg1: *mut sqlite3) -> *const libc::c_char;
    pub fn sqlite3_errmsg16(arg1: *mut sqlite3) -> *const libc::c_void;
    pub fn sqlite3_errstr(arg1: libc::c_int) -> *const libc::c_char;
    pub fn sqlite3_limit(arg1: *mut sqlite3, id: libc::c_int,
                         newVal: libc::c_int) -> libc::c_int;
    pub fn sqlite3_prepare(db: *mut sqlite3, zSql: *const libc::c_char,
                           nByte: libc::c_int,
                           ppStmt: *mut *mut sqlite3_stmt,
                           pzTail: *mut *const libc::c_char)
     -> libc::c_int;
    pub fn sqlite3_prepare_v2(db: *mut sqlite3, zSql: *const libc::c_char,
                              nByte: libc::c_int,
                              ppStmt: *mut *mut sqlite3_stmt,
                              pzTail: *mut *const libc::c_char)
     -> libc::c_int;
    pub fn sqlite3_prepare16(db: *mut sqlite3, zSql: *const libc::c_void,
                             nByte: libc::c_int,
                             ppStmt: *mut *mut sqlite3_stmt,
                             pzTail: *mut *const libc::c_void)
     -> libc::c_int;
    pub fn sqlite3_prepare16_v2(db: *mut sqlite3, zSql: *const libc::c_void,
                                nByte: libc::c_int,
                                ppStmt: *mut *mut sqlite3_stmt,
                                pzTail: *mut *const libc::c_void)
     -> libc::c_int;
    pub fn sqlite3_sql(pStmt: *mut sqlite3_stmt) -> *const libc::c_char;
    pub fn sqlite3_stmt_readonly(pStmt: *mut sqlite3_stmt) -> libc::c_int;
    pub fn sqlite3_stmt_busy(arg1: *mut sqlite3_stmt) -> libc::c_int;
    pub fn sqlite3_bind_blob(arg1: *mut sqlite3_stmt, arg2: libc::c_int,
                             arg3: *const libc::c_void, n: libc::c_int,
                             arg4:
                                 ::std::option::Option<extern "C" fn(arg1:
                                                                         *mut libc::c_void)
                                                           -> ()>)
     -> libc::c_int;
    pub fn sqlite3_bind_blob64(arg1: *mut sqlite3_stmt, arg2: libc::c_int,
                               arg3: *const libc::c_void,
                               arg4: sqlite3_uint64,
                               arg5:
                                   ::std::option::Option<extern "C" fn(arg1:
                                                                           *mut libc::c_void)
                                                             -> ()>)
     -> libc::c_int;
    pub fn sqlite3_bind_double(arg1: *mut sqlite3_stmt, arg2: libc::c_int,
                               arg3: libc::c_double) -> libc::c_int;
    pub fn sqlite3_bind_int(arg1: *mut sqlite3_stmt, arg2: libc::c_int,
                            arg3: libc::c_int) -> libc::c_int;
    pub fn sqlite3_bind_int64(arg1: *mut sqlite3_stmt, arg2: libc::c_int,
                              arg3: sqlite3_int64) -> libc::c_int;
    pub fn sqlite3_bind_null(arg1: *mut sqlite3_stmt, arg2: libc::c_int)
     -> libc::c_int;
    pub fn sqlite3_bind_text(arg1: *mut sqlite3_stmt, arg2: libc::c_int,
                             arg3: *const libc::c_char, arg4: libc::c_int,
                             arg5:
                                 ::std::option::Option<extern "C" fn(arg1:
                                                                         *mut libc::c_void)
                                                           -> ()>)
     -> libc::c_int;
    pub fn sqlite3_bind_text16(arg1: *mut sqlite3_stmt, arg2: libc::c_int,
                               arg3: *const libc::c_void,
                               arg4: libc::c_int,
                               arg5:
                                   ::std::option::Option<extern "C" fn(arg1:
                                                                           *mut libc::c_void)
                                                             -> ()>)
     -> libc::c_int;
    pub fn sqlite3_bind_text64(arg1: *mut sqlite3_stmt, arg2: libc::c_int,
                               arg3: *const libc::c_char,
                               arg4: sqlite3_uint64,
                               arg5:
                                   ::std::option::Option<extern "C" fn(arg1:
                                                                           *mut libc::c_void)
                                                             -> ()>,
                               encoding: libc::c_uchar) -> libc::c_int;
    pub fn sqlite3_bind_value(arg1: *mut sqlite3_stmt, arg2: libc::c_int,
                              arg3: *const sqlite3_value) -> libc::c_int;
    pub fn sqlite3_bind_zeroblob(arg1: *mut sqlite3_stmt, arg2: libc::c_int,
                                 n: libc::c_int) -> libc::c_int;
    pub fn sqlite3_bind_parameter_count(arg1: *mut sqlite3_stmt)
     -> libc::c_int;
    pub fn sqlite3_bind_parameter_name(arg1: *mut sqlite3_stmt,
                                       arg2: libc::c_int)
     -> *const libc::c_char;
    pub fn sqlite3_bind_parameter_index(arg1: *mut sqlite3_stmt,
                                        zName: *const libc::c_char)
     -> libc::c_int;
    pub fn sqlite3_clear_bindings(arg1: *mut sqlite3_stmt) -> libc::c_int;
    pub fn sqlite3_column_count(pStmt: *mut sqlite3_stmt) -> libc::c_int;
    pub fn sqlite3_column_name(arg1: *mut sqlite3_stmt, N: libc::c_int)
     -> *const libc::c_char;
    pub fn sqlite3_column_name16(arg1: *mut sqlite3_stmt, N: libc::c_int)
     -> *const libc::c_void;
    pub fn sqlite3_column_database_name(arg1: *mut sqlite3_stmt,
                                        arg2: libc::c_int)
     -> *const libc::c_char;
    pub fn sqlite3_column_database_name16(arg1: *mut sqlite3_stmt,
                                          arg2: libc::c_int)
     -> *const libc::c_void;
    pub fn sqlite3_column_table_name(arg1: *mut sqlite3_stmt,
                                     arg2: libc::c_int)
     -> *const libc::c_char;
    pub fn sqlite3_column_table_name16(arg1: *mut sqlite3_stmt,
                                       arg2: libc::c_int)
     -> *const libc::c_void;
    pub fn sqlite3_column_origin_name(arg1: *mut sqlite3_stmt,
                                      arg2: libc::c_int)
     -> *const libc::c_char;
    pub fn sqlite3_column_origin_name16(arg1: *mut sqlite3_stmt,
                                        arg2: libc::c_int)
     -> *const libc::c_void;
    pub fn sqlite3_column_decltype(arg1: *mut sqlite3_stmt,
                                   arg2: libc::c_int)
     -> *const libc::c_char;
    pub fn sqlite3_column_decltype16(arg1: *mut sqlite3_stmt,
                                     arg2: libc::c_int)
     -> *const libc::c_void;
    pub fn sqlite3_step(arg1: *mut sqlite3_stmt) -> libc::c_int;
    pub fn sqlite3_data_count(pStmt: *mut sqlite3_stmt) -> libc::c_int;
    pub fn sqlite3_column_blob(arg1: *mut sqlite3_stmt, iCol: libc::c_int)
     -> *const libc::c_void;
    pub fn sqlite3_column_bytes(arg1: *mut sqlite3_stmt, iCol: libc::c_int)
     -> libc::c_int;
    pub fn sqlite3_column_bytes16(arg1: *mut sqlite3_stmt,
                                  iCol: libc::c_int) -> libc::c_int;
    pub fn sqlite3_column_double(arg1: *mut sqlite3_stmt, iCol: libc::c_int)
     -> libc::c_double;
    pub fn sqlite3_column_int(arg1: *mut sqlite3_stmt, iCol: libc::c_int)
     -> libc::c_int;
    pub fn sqlite3_column_int64(arg1: *mut sqlite3_stmt, iCol: libc::c_int)
     -> sqlite3_int64;
    pub fn sqlite3_column_text(arg1: *mut sqlite3_stmt, iCol: libc::c_int)
     -> *const libc::c_uchar;
    pub fn sqlite3_column_text16(arg1: *mut sqlite3_stmt, iCol: libc::c_int)
     -> *const libc::c_void;
    pub fn sqlite3_column_type(arg1: *mut sqlite3_stmt, iCol: libc::c_int)
     -> libc::c_int;
    pub fn sqlite3_column_value(arg1: *mut sqlite3_stmt, iCol: libc::c_int)
     -> *mut sqlite3_value;
    pub fn sqlite3_finalize(pStmt: *mut sqlite3_stmt) -> libc::c_int;
    pub fn sqlite3_reset(pStmt: *mut sqlite3_stmt) -> libc::c_int;
    pub fn sqlite3_create_function(db: *mut sqlite3,
                                   zFunctionName: *const libc::c_char,
                                   nArg: libc::c_int,
                                   eTextRep: libc::c_int,
                                   pApp: *mut libc::c_void,
                                   xFunc:
                                       ::std::option::Option<extern "C" fn(arg1:
                                                                               *mut sqlite3_context,
                                                                           arg2:
                                                                               libc::c_int,
                                                                           arg3:
                                                                               *mut *mut sqlite3_value)
                                                                 -> ()>,
                                   xStep:
                                       ::std::option::Option<extern "C" fn(arg1:
                                                                               *mut sqlite3_context,
                                                                           arg2:
                                                                               libc::c_int,
                                                                           arg3:
                                                                               *mut *mut sqlite3_value)
                                                                 -> ()>,
                                   xFinal:
                                       ::std::option::Option<extern "C" fn(arg1:
                                                                               *mut sqlite3_context)
                                                                 -> ()>)
     -> libc::c_int;
    pub fn sqlite3_create_function16(db: *mut sqlite3,
                                     zFunctionName: *const libc::c_void,
                                     nArg: libc::c_int,
                                     eTextRep: libc::c_int,
                                     pApp: *mut libc::c_void,
                                     xFunc:
                                         ::std::option::Option<extern "C" fn(arg1:
                                                                                 *mut sqlite3_context,
                                                                             arg2:
                                                                                 libc::c_int,
                                                                             arg3:
                                                                                 *mut *mut sqlite3_value)
                                                                   -> ()>,
                                     xStep:
                                         ::std::option::Option<extern "C" fn(arg1:
                                                                                 *mut sqlite3_context,
                                                                             arg2:
                                                                                 libc::c_int,
                                                                             arg3:
                                                                                 *mut *mut sqlite3_value)
                                                                   -> ()>,
                                     xFinal:
                                         ::std::option::Option<extern "C" fn(arg1:
                                                                                 *mut sqlite3_context)
                                                                   -> ()>)
     -> libc::c_int;
    pub fn sqlite3_create_function_v2(db: *mut sqlite3,
                                      zFunctionName: *const libc::c_char,
                                      nArg: libc::c_int,
                                      eTextRep: libc::c_int,
                                      pApp: *mut libc::c_void,
                                      xFunc:
                                          ::std::option::Option<extern "C" fn(arg1:
                                                                                  *mut sqlite3_context,
                                                                              arg2:
                                                                                  libc::c_int,
                                                                              arg3:
                                                                                  *mut *mut sqlite3_value)
                                                                    -> ()>,
                                      xStep:
                                          ::std::option::Option<extern "C" fn(arg1:
                                                                                  *mut sqlite3_context,
                                                                              arg2:
                                                                                  libc::c_int,
                                                                              arg3:
                                                                                  *mut *mut sqlite3_value)
                                                                    -> ()>,
                                      xFinal:
                                          ::std::option::Option<extern "C" fn(arg1:
                                                                                  *mut sqlite3_context)
                                                                    -> ()>,
                                      xDestroy:
                                          ::std::option::Option<extern "C" fn(arg1:
                                                                                  *mut libc::c_void)
                                                                    -> ()>)
     -> libc::c_int;
    pub fn sqlite3_aggregate_count(arg1: *mut sqlite3_context)
     -> libc::c_int;
    pub fn sqlite3_expired(arg1: *mut sqlite3_stmt) -> libc::c_int;
    pub fn sqlite3_transfer_bindings(arg1: *mut sqlite3_stmt,
                                     arg2: *mut sqlite3_stmt)
     -> libc::c_int;
    pub fn sqlite3_global_recover() -> libc::c_int;
    pub fn sqlite3_thread_cleanup() -> ();
    pub fn sqlite3_memory_alarm(arg1:
                                    ::std::option::Option<extern "C" fn(arg1:
                                                                            *mut libc::c_void,
                                                                        arg2:
                                                                            sqlite3_int64,
                                                                        arg3:
                                                                            libc::c_int)
                                                              -> ()>,
                                arg2: *mut libc::c_void,
                                arg3: sqlite3_int64) -> libc::c_int;
    pub fn sqlite3_value_blob(arg1: *mut sqlite3_value)
     -> *const libc::c_void;
    pub fn sqlite3_value_bytes(arg1: *mut sqlite3_value) -> libc::c_int;
    pub fn sqlite3_value_bytes16(arg1: *mut sqlite3_value) -> libc::c_int;
    pub fn sqlite3_value_double(arg1: *mut sqlite3_value) -> libc::c_double;
    pub fn sqlite3_value_int(arg1: *mut sqlite3_value) -> libc::c_int;
    pub fn sqlite3_value_int64(arg1: *mut sqlite3_value) -> sqlite3_int64;
    pub fn sqlite3_value_text(arg1: *mut sqlite3_value)
     -> *const libc::c_uchar;
    pub fn sqlite3_value_text16(arg1: *mut sqlite3_value)
     -> *const libc::c_void;
    pub fn sqlite3_value_text16le(arg1: *mut sqlite3_value)
     -> *const libc::c_void;
    pub fn sqlite3_value_text16be(arg1: *mut sqlite3_value)
     -> *const libc::c_void;
    pub fn sqlite3_value_type(arg1: *mut sqlite3_value) -> libc::c_int;
    pub fn sqlite3_value_numeric_type(arg1: *mut sqlite3_value)
     -> libc::c_int;
    pub fn sqlite3_aggregate_context(arg1: *mut sqlite3_context,
                                     nBytes: libc::c_int)
     -> *mut libc::c_void;
    pub fn sqlite3_user_data(arg1: *mut sqlite3_context)
     -> *mut libc::c_void;
    pub fn sqlite3_context_db_handle(arg1: *mut sqlite3_context)
     -> *mut sqlite3;
    pub fn sqlite3_get_auxdata(arg1: *mut sqlite3_context, N: libc::c_int)
     -> *mut libc::c_void;
    pub fn sqlite3_set_auxdata(arg1: *mut sqlite3_context, N: libc::c_int,
                               arg2: *mut libc::c_void,
                               arg3:
                                   ::std::option::Option<extern "C" fn(arg1:
                                                                           *mut libc::c_void)
                                                             -> ()>) -> ();
    pub fn sqlite3_result_blob(arg1: *mut sqlite3_context,
                               arg2: *const libc::c_void,
                               arg3: libc::c_int,
                               arg4:
                                   ::std::option::Option<extern "C" fn(arg1:
                                                                           *mut libc::c_void)
                                                             -> ()>) -> ();
    pub fn sqlite3_result_blob64(arg1: *mut sqlite3_context,
                                 arg2: *const libc::c_void,
                                 arg3: sqlite3_uint64,
                                 arg4:
                                     ::std::option::Option<extern "C" fn(arg1:
                                                                             *mut libc::c_void)
                                                               -> ()>) -> ();
    pub fn sqlite3_result_double(arg1: *mut sqlite3_context,
                                 arg2: libc::c_double) -> ();
    pub fn sqlite3_result_error(arg1: *mut sqlite3_context,
                                arg2: *const libc::c_char,
                                arg3: libc::c_int) -> ();
    pub fn sqlite3_result_error16(arg1: *mut sqlite3_context,
                                  arg2: *const libc::c_void,
                                  arg3: libc::c_int) -> ();
    pub fn sqlite3_result_error_toobig(arg1: *mut sqlite3_context) -> ();
    pub fn sqlite3_result_error_nomem(arg1: *mut sqlite3_context) -> ();
    pub fn sqlite3_result_error_code(arg1: *mut sqlite3_context,
                                     arg2: libc::c_int) -> ();
    pub fn sqlite3_result_int(arg1: *mut sqlite3_context, arg2: libc::c_int)
     -> ();
    pub fn sqlite3_result_int64(arg1: *mut sqlite3_context,
                                arg2: sqlite3_int64) -> ();
    pub fn sqlite3_result_null(arg1: *mut sqlite3_context) -> ();
    pub fn sqlite3_result_text(arg1: *mut sqlite3_context,
                               arg2: *const libc::c_char,
                               arg3: libc::c_int,
                               arg4:
                                   ::std::option::Option<extern "C" fn(arg1:
                                                                           *mut libc::c_void)
                                                             -> ()>) -> ();
    pub fn sqlite3_result_text64(arg1: *mut sqlite3_context,
                                 arg2: *const libc::c_char,
                                 arg3: sqlite3_uint64,
                                 arg4:
                                     ::std::option::Option<extern "C" fn(arg1:
                                                                             *mut libc::c_void)
                                                               -> ()>,
                                 encoding: libc::c_uchar) -> ();
    pub fn sqlite3_result_text16(arg1: *mut sqlite3_context,
                                 arg2: *const libc::c_void,
                                 arg3: libc::c_int,
                                 arg4:
                                     ::std::option::Option<extern "C" fn(arg1:
                                                                             *mut libc::c_void)
                                                               -> ()>) -> ();
    pub fn sqlite3_result_text16le(arg1: *mut sqlite3_context,
                                   arg2: *const libc::c_void,
                                   arg3: libc::c_int,
                                   arg4:
                                       ::std::option::Option<extern "C" fn(arg1:
                                                                               *mut libc::c_void)
                                                                 -> ()>)
     -> ();
    pub fn sqlite3_result_text16be(arg1: *mut sqlite3_context,
                                   arg2: *const libc::c_void,
                                   arg3: libc::c_int,
                                   arg4:
                                       ::std::option::Option<extern "C" fn(arg1:
                                                                               *mut libc::c_void)
                                                                 -> ()>)
     -> ();
    pub fn sqlite3_result_value(arg1: *mut sqlite3_context,
                                arg2: *mut sqlite3_value) -> ();
    pub fn sqlite3_result_zeroblob(arg1: *mut sqlite3_context,
                                   n: libc::c_int) -> ();
    pub fn sqlite3_create_collation(arg1: *mut sqlite3,
                                    zName: *const libc::c_char,
                                    eTextRep: libc::c_int,
                                    pArg: *mut libc::c_void,
                                    xCompare:
                                        ::std::option::Option<extern "C" fn(arg1:
                                                                                *mut libc::c_void,
                                                                            arg2:
                                                                                libc::c_int,
                                                                            arg3:
                                                                                *const libc::c_void,
                                                                            arg4:
                                                                                libc::c_int,
                                                                            arg5:
                                                                                *const libc::c_void)
                                                                  ->
                                                                      libc::c_int>)
     -> libc::c_int;
    pub fn sqlite3_create_collation_v2(arg1: *mut sqlite3,
                                       zName: *const libc::c_char,
                                       eTextRep: libc::c_int,
                                       pArg: *mut libc::c_void,
                                       xCompare:
                                           ::std::option::Option<extern "C" fn(arg1:
                                                                                   *mut libc::c_void,
                                                                               arg2:
                                                                                   libc::c_int,
                                                                               arg3:
                                                                                   *const libc::c_void,
                                                                               arg4:
                                                                                   libc::c_int,
                                                                               arg5:
                                                                                   *const libc::c_void)
                                                                     ->
                                                                         libc::c_int>,
                                       xDestroy:
                                           ::std::option::Option<extern "C" fn(arg1:
                                                                                   *mut libc::c_void)
                                                                     -> ()>)
     -> libc::c_int;
    pub fn sqlite3_create_collation16(arg1: *mut sqlite3,
                                      zName: *const libc::c_void,
                                      eTextRep: libc::c_int,
                                      pArg: *mut libc::c_void,
                                      xCompare:
                                          ::std::option::Option<extern "C" fn(arg1:
                                                                                  *mut libc::c_void,
                                                                              arg2:
                                                                                  libc::c_int,
                                                                              arg3:
                                                                                  *const libc::c_void,
                                                                              arg4:
                                                                                  libc::c_int,
                                                                              arg5:
                                                                                  *const libc::c_void)
                                                                    ->
                                                                        libc::c_int>)
     -> libc::c_int;
    pub fn sqlite3_collation_needed(arg1: *mut sqlite3,
                                    arg2: *mut libc::c_void,
                                    arg3:
                                        ::std::option::Option<extern "C" fn(arg1:
                                                                                *mut libc::c_void,
                                                                            arg2:
                                                                                *mut sqlite3,
                                                                            eTextRep:
                                                                                libc::c_int,
                                                                            arg3:
                                                                                *const libc::c_char)
                                                                  -> ()>)
     -> libc::c_int;
    pub fn sqlite3_collation_needed16(arg1: *mut sqlite3,
                                      arg2: *mut libc::c_void,
                                      arg3:
                                          ::std::option::Option<extern "C" fn(arg1:
                                                                                  *mut libc::c_void,
                                                                              arg2:
                                                                                  *mut sqlite3,
                                                                              eTextRep:
                                                                                  libc::c_int,
                                                                              arg3:
                                                                                  *const libc::c_void)
                                                                    -> ()>)
     -> libc::c_int;
    pub fn sqlite3_sleep(arg1: libc::c_int) -> libc::c_int;
    pub fn sqlite3_get_autocommit(arg1: *mut sqlite3) -> libc::c_int;
    pub fn sqlite3_db_handle(arg1: *mut sqlite3_stmt) -> *mut sqlite3;
    pub fn sqlite3_db_filename(db: *mut sqlite3,
                               zDbName: *const libc::c_char)
     -> *const libc::c_char;
    pub fn sqlite3_db_readonly(db: *mut sqlite3,
                               zDbName: *const libc::c_char)
     -> libc::c_int;
    pub fn sqlite3_next_stmt(pDb: *mut sqlite3, pStmt: *mut sqlite3_stmt)
     -> *mut sqlite3_stmt;
    pub fn sqlite3_commit_hook(arg1: *mut sqlite3,
                               arg2:
                                   ::std::option::Option<extern "C" fn(arg1:
                                                                           *mut libc::c_void)
                                                             ->
                                                                 libc::c_int>,
                               arg3: *mut libc::c_void)
     -> *mut libc::c_void;
    pub fn sqlite3_rollback_hook(arg1: *mut sqlite3,
                                 arg2:
                                     ::std::option::Option<extern "C" fn(arg1:
                                                                             *mut libc::c_void)
                                                               -> ()>,
                                 arg3: *mut libc::c_void)
     -> *mut libc::c_void;
    pub fn sqlite3_update_hook(arg1: *mut sqlite3,
                               arg2:
                                   ::std::option::Option<extern "C" fn(arg1:
                                                                           *mut libc::c_void,
                                                                       arg2:
                                                                           libc::c_int,
                                                                       arg3:
                                                                           *const libc::c_char,
                                                                       arg4:
                                                                           *const libc::c_char,
                                                                       arg5:
                                                                           sqlite3_int64)
                                                             -> ()>,
                               arg3: *mut libc::c_void)
     -> *mut libc::c_void;
    pub fn sqlite3_enable_shared_cache(arg1: libc::c_int) -> libc::c_int;
    pub fn sqlite3_release_memory(arg1: libc::c_int) -> libc::c_int;
    pub fn sqlite3_db_release_memory(arg1: *mut sqlite3) -> libc::c_int;
    pub fn sqlite3_soft_heap_limit64(N: sqlite3_int64) -> sqlite3_int64;
    pub fn sqlite3_soft_heap_limit(N: libc::c_int) -> ();
    pub fn sqlite3_table_column_metadata(db: *mut sqlite3,
                                         zDbName: *const libc::c_char,
                                         zTableName: *const libc::c_char,
                                         zColumnName: *const libc::c_char,
                                         pzDataType:
                                             *mut *const libc::c_char,
                                         pzCollSeq:
                                             *mut *const libc::c_char,
                                         pNotNull: *mut libc::c_int,
                                         pPrimaryKey: *mut libc::c_int,
                                         pAutoinc: *mut libc::c_int)
     -> libc::c_int;
    pub fn sqlite3_load_extension(db: *mut sqlite3,
                                  zFile: *const libc::c_char,
                                  zProc: *const libc::c_char,
                                  pzErrMsg: *mut *mut libc::c_char)
     -> libc::c_int;
    pub fn sqlite3_enable_load_extension(db: *mut sqlite3,
                                         onoff: libc::c_int)
     -> libc::c_int;
    pub fn sqlite3_auto_extension(xEntryPoint:
                                      ::std::option::Option<extern "C" fn()
                                                                -> ()>)
     -> libc::c_int;
    pub fn sqlite3_cancel_auto_extension(xEntryPoint:
                                             ::std::option::Option<extern "C" fn()
                                                                       -> ()>)
     -> libc::c_int;
    pub fn sqlite3_reset_auto_extension() -> ();
    pub fn sqlite3_create_module(db: *mut sqlite3,
                                 zName: *const libc::c_char,
                                 p: *const sqlite3_module,
                                 pClientData: *mut libc::c_void)
     -> libc::c_int;
    pub fn sqlite3_create_module_v2(db: *mut sqlite3,
                                    zName: *const libc::c_char,
                                    p: *const sqlite3_module,
                                    pClientData: *mut libc::c_void,
                                    xDestroy:
                                        ::std::option::Option<extern "C" fn(arg1:
                                                                                *mut libc::c_void)
                                                                  -> ()>)
     -> libc::c_int;
    pub fn sqlite3_declare_vtab(arg1: *mut sqlite3,
                                zSQL: *const libc::c_char) -> libc::c_int;
    pub fn sqlite3_overload_function(arg1: *mut sqlite3,
                                     zFuncName: *const libc::c_char,
                                     nArg: libc::c_int) -> libc::c_int;
    pub fn sqlite3_blob_open(arg1: *mut sqlite3, zDb: *const libc::c_char,
                             zTable: *const libc::c_char,
                             zColumn: *const libc::c_char,
                             iRow: sqlite3_int64, flags: libc::c_int,
                             ppBlob: *mut *mut sqlite3_blob) -> libc::c_int;
    pub fn sqlite3_blob_reopen(arg1: *mut sqlite3_blob, arg2: sqlite3_int64)
     -> libc::c_int;
    pub fn sqlite3_blob_close(arg1: *mut sqlite3_blob) -> libc::c_int;
    pub fn sqlite3_blob_bytes(arg1: *mut sqlite3_blob) -> libc::c_int;
    pub fn sqlite3_blob_read(arg1: *mut sqlite3_blob, Z: *mut libc::c_void,
                             N: libc::c_int, iOffset: libc::c_int)
     -> libc::c_int;
    pub fn sqlite3_blob_write(arg1: *mut sqlite3_blob,
                              z: *const libc::c_void, n: libc::c_int,
                              iOffset: libc::c_int) -> libc::c_int;
    pub fn sqlite3_vfs_find(zVfsName: *const libc::c_char)
     -> *mut sqlite3_vfs;
    pub fn sqlite3_vfs_register(arg1: *mut sqlite3_vfs,
                                makeDflt: libc::c_int) -> libc::c_int;
    pub fn sqlite3_vfs_unregister(arg1: *mut sqlite3_vfs) -> libc::c_int;
    pub fn sqlite3_mutex_alloc(arg1: libc::c_int) -> *mut sqlite3_mutex;
    pub fn sqlite3_mutex_free(arg1: *mut sqlite3_mutex) -> ();
    pub fn sqlite3_mutex_enter(arg1: *mut sqlite3_mutex) -> ();
    pub fn sqlite3_mutex_try(arg1: *mut sqlite3_mutex) -> libc::c_int;
    pub fn sqlite3_mutex_leave(arg1: *mut sqlite3_mutex) -> ();
    pub fn sqlite3_mutex_held(arg1: *mut sqlite3_mutex) -> libc::c_int;
    pub fn sqlite3_mutex_notheld(arg1: *mut sqlite3_mutex) -> libc::c_int;
    pub fn sqlite3_db_mutex(arg1: *mut sqlite3) -> *mut sqlite3_mutex;
    pub fn sqlite3_file_control(arg1: *mut sqlite3,
                                zDbName: *const libc::c_char,
                                op: libc::c_int, arg2: *mut libc::c_void)
     -> libc::c_int;
    pub fn sqlite3_test_control(op: libc::c_int, ...) -> libc::c_int;
    pub fn sqlite3_status(op: libc::c_int, pCurrent: *mut libc::c_int,
                          pHighwater: *mut libc::c_int,
                          resetFlag: libc::c_int) -> libc::c_int;
    pub fn sqlite3_status64(op: libc::c_int, pCurrent: *mut sqlite3_int64,
                            pHighwater: *mut sqlite3_int64,
                            resetFlag: libc::c_int) -> libc::c_int;
    pub fn sqlite3_db_status(arg1: *mut sqlite3, op: libc::c_int,
                             pCur: *mut libc::c_int,
                             pHiwtr: *mut libc::c_int,
                             resetFlg: libc::c_int) -> libc::c_int;
    pub fn sqlite3_stmt_status(arg1: *mut sqlite3_stmt, op: libc::c_int,
                               resetFlg: libc::c_int) -> libc::c_int;
    pub fn sqlite3_backup_init(pDest: *mut sqlite3,
                               zDestName: *const libc::c_char,
                               pSource: *mut sqlite3,
                               zSourceName: *const libc::c_char)
     -> *mut sqlite3_backup;
    pub fn sqlite3_backup_step(p: *mut sqlite3_backup, nPage: libc::c_int)
     -> libc::c_int;
    pub fn sqlite3_backup_finish(p: *mut sqlite3_backup) -> libc::c_int;
    pub fn sqlite3_backup_remaining(p: *mut sqlite3_backup) -> libc::c_int;
    pub fn sqlite3_backup_pagecount(p: *mut sqlite3_backup) -> libc::c_int;
    pub fn sqlite3_unlock_notify(pBlocked: *mut sqlite3,
                                 xNotify:
                                     ::std::option::Option<extern "C" fn(apArg:
                                                                             *mut *mut libc::c_void,
                                                                         nArg:
                                                                             libc::c_int)
                                                               -> ()>,
                                 pNotifyArg: *mut libc::c_void)
     -> libc::c_int;
    pub fn sqlite3_stricmp(arg1: *const libc::c_char,
                           arg2: *const libc::c_char) -> libc::c_int;
    pub fn sqlite3_strnicmp(arg1: *const libc::c_char,
                            arg2: *const libc::c_char, arg3: libc::c_int)
     -> libc::c_int;
    pub fn sqlite3_strglob(zGlob: *const libc::c_char,
                           zStr: *const libc::c_char) -> libc::c_int;
    pub fn sqlite3_log(iErrCode: libc::c_int,
                       zFormat: *const libc::c_char, ...) -> ();
    pub fn sqlite3_wal_hook(arg1: *mut sqlite3,
                            arg2:
                                ::std::option::Option<extern "C" fn(arg1:
                                                                        *mut libc::c_void,
                                                                    arg2:
                                                                        *mut sqlite3,
                                                                    arg3:
                                                                        *const libc::c_char,
                                                                    arg4:
                                                                        libc::c_int)
                                                          -> libc::c_int>,
                            arg3: *mut libc::c_void) -> *mut libc::c_void;
    pub fn sqlite3_wal_autocheckpoint(db: *mut sqlite3, N: libc::c_int)
     -> libc::c_int;
    pub fn sqlite3_wal_checkpoint(db: *mut sqlite3,
                                  zDb: *const libc::c_char)
     -> libc::c_int;
    pub fn sqlite3_wal_checkpoint_v2(db: *mut sqlite3,
                                     zDb: *const libc::c_char,
                                     eMode: libc::c_int,
                                     pnLog: *mut libc::c_int,
                                     pnCkpt: *mut libc::c_int)
     -> libc::c_int;
    pub fn sqlite3_vtab_config(arg1: *mut sqlite3, op: libc::c_int, ...)
     -> libc::c_int;
    pub fn sqlite3_vtab_on_conflict(arg1: *mut sqlite3) -> libc::c_int;
    pub fn sqlite3_stmt_scanstatus(pStmt: *mut sqlite3_stmt,
                                   idx: libc::c_int,
                                   iScanStatusOp: libc::c_int,
                                   pOut: *mut libc::c_void)
     -> libc::c_int;
    pub fn sqlite3_stmt_scanstatus_reset(arg1: *mut sqlite3_stmt) -> ();
    pub fn sqlite3_rtree_geometry_callback(db: *mut sqlite3,
                                           zGeom: *const libc::c_char,
                                           xGeom:
                                               ::std::option::Option<extern "C" fn(arg1:
                                                                                       *mut sqlite3_rtree_geometry,
                                                                                   arg2:
                                                                                       libc::c_int,
                                                                                   arg3:
                                                                                       *mut sqlite3_rtree_dbl,
                                                                                   arg4:
                                                                                       *mut libc::c_int)
                                                                         ->
                                                                             libc::c_int>,
                                           pContext: *mut libc::c_void)
     -> libc::c_int;
    pub fn sqlite3_rtree_query_callback(db: *mut sqlite3,
                                        zQueryFunc: *const libc::c_char,
                                        xQueryFunc:
                                            ::std::option::Option<extern "C" fn(arg1:
                                                                                    *mut sqlite3_rtree_query_info)
                                                                      ->
                                                                          libc::c_int>,
                                        pContext: *mut libc::c_void,
                                        xDestructor:
                                            ::std::option::Option<extern "C" fn(arg1:
                                                                                    *mut libc::c_void)
                                                                      -> ()>)
     -> libc::c_int;
}
