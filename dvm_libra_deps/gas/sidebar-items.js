initSidebarItems({"constant":[["CONST_SIZE","The size in bytes for a non-string or address constant on the stack"],["DEFAULT_ACCOUNT_SIZE","For V1 all accounts will be ~800 bytes"],["LARGE_TRANSACTION_CUTOFF","Any transaction over this size will be charged `INTRINSIC_GAS_PER_BYTE` per byte"],["MAX_ABSTRACT_MEMORY_SIZE","The maximum size representable by AbstractMemorySize"],["MIN_EXISTS_DATA_SIZE","For exists checks on data that doesn't exists this is the multiplier that is used."],["ONE_GAS_UNIT","One unit of gas"],["REFERENCE_SIZE","The size in bytes for a reference on the stack"],["STRUCT_SIZE","The size of a struct in bytes"]],"enum":[["N",""],["NativeCostIndex",""]],"fn":[["calculate_intrinsic_gas","Calculate the intrinsic gas for the transaction based upon its size in bytes/words."],["new_from_instructions",""],["zero_cost_schedule",""]],"struct":[["AbstractMemorySize","A newtype wrapper that represents the (abstract) memory size that the instruction will take up."],["CostStrategy","The Move VM implementation for gas charging."],["CostTable","The cost tables, keyed by the serialized form of the bytecode instruction.  We use the serialized form as opposed to the instruction enum itself as the key since this will be the on-chain representation of bytecode instructions in the future."],["GasConstants",""],["GasCost","The  `GasCost` tracks: - instruction cost: how much time/computational power is needed to perform the instruction - memory cost: how much memory is required for the instruction, and storage overhead"],["GasPrice","A newtype wrapper around the gas price for each unit of gas consumed."],["GasUnits","A newtype wrapper around the underlying carrier for the gas cost."]],"trait":[["GasAlgebra","A trait encoding the operations permitted on the underlying carrier for the gas unit, and how other gas-related units can interact with other units -- operations can only be performed across units with the same underlying carrier (i.e. as long as the underlying data is the same)."]],"type":[["GasCarrier","The underlying carrier for gas-related units and costs. Data with this type should not be manipulated directly, but instead be manipulated using the newtype wrappers defined around them and the functions defined in the `GasAlgebra` trait."]]});