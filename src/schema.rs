// The order of the fields here must match
// the column order in the corresponding table.
table! {
    dogs (id) {
        id -> Int4,
        breed -> Nullable<Text>,
        name -> Nullable<Text>,
    }
}
