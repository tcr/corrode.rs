/* ERROR: cannot yet convert file "./corrode/src/Language/Rust/AST.hs"
Error: Unrecognized token `where`:
 38 |           ;LitBool b -> text $ if b then "dHJ1ZQ==" else "ZmFsc2U="
 39 |           ;LitInt i repr (TypeName ty) -> text $ s ++ ty
 40 |               where
~~~~~~~~~~~~~~~~~~~~^
*/
/* ERROR: cannot yet convert file "./corrode/src/Language/Rust/Corrode/C.lhs"
Error: Unrecognized token `where`:
267 |     {Left msg -> Left msg
268 |      ;Right _ -> Right items'
269 |      ;where {initFlow = FunctionContext
~~~~~~~~~~~~^
*/
/* ERROR: cannot yet convert file "./corrode/src/Language/Rust/Corrode/CFG.lhs"
Error: Unrecognized token `;`:
 49 |
 50 |  };data Unordered
 51 | ;data DepthFirst
~~~~~~^
*/
mod Language_Rust_Corrode_CrateMap {
    #[derive(Eq, Ord, Show)]
    struct ItemKind(Enum, Struct, Union, Type, Symbol);

    fn mergeCrateMaps() -> Map.Map(String, CrateMap) {
        Map.fromListWith((Map.unionWith((Operator("++")))))
    }

    fn parseCrateMap() -> Either(String, CrateMap) {
        (fmap(root) . (foldrM(parseLine, (Map.empty, vec![])) . (filter(((not . null))) . (map(cleanLine) . lines))))
    }

    fn rewritesFromCratesMap(crates: CratesMap) -> ItemRewrites {
        Map.fromList(Dummy)
    }

    fn splitModuleMap(modName: String, crates: CratesMap) -> (ModuleMap, CratesMap) {
        fromMaybe((vec![], crates))({
                let thisCrate = Map.lookup("".to_string(), crates);
                let thisModule = Map.lookup(modName, thisCrate);
                Let;
                Let;
                return((thisModule, crates'))
            })
    }

}

mod Language_Rust_Idiomatic {
    fn itemIdioms(__0: Rust.Item) -> Rust.Item {
        match (__0) {
            Rust.Item(attrs, vis, Rust.Function(fattrs, name, formals, ret, b)) => Rust.Item(attrs, vis, (Rust.Function(fattrs, name, formals, ret, (tailBlock(b))))),
            i => i,
        }
    }

    fn tailBlock(__0: Rust.Block) -> Rust.Block {
        match (__0) {
            Rust.Block(b, Just((tailExpr -> Just(e)))) => Rust.Block(b, e),
            Rust.Block((unsnoc -> Just((b, Rust.Stmt((tailExpr -> Just(e)))))), Nothing) => Rust.Block(b, e),
            b => b,
        }
    }

    fn tailExpr(__0: Rust.Expr) -> Maybe(Maybe(Rust.Expr)) {
        match (__0) {
            Rust.Return(e) => Just(e),
            Rust.BlockExpr(b) => Just((Just((Rust.BlockExpr((tailBlock(b))))))),
            Rust.IfThenElse(c, t, f) => Just((Just((Rust.IfThenElse(c, (tailBlock(t)), (tailBlock(f))))))),
            _ => Nothing,
        }
    }

    fn unsnoc(__0: Vec<a>) -> Maybe((Vec<a>, a)) {
        match (__0) {
            [] => Nothing,
            x(<todo>, xs) => match unsnoc(xs) {
                    Just, (a, b) => Just((:(x, a), b)),
                    Nothing => Just((vec![], x)),
                },
        }
    }

}

mod Language_Rust {

}



fn main() { /* demo */ }
