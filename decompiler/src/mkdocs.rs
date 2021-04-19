use std::fs::File;
use std::io::{Write, BufWriter};
use std::rc::Rc;
//use std::str::FromStr;
use std::path::Path;

//use redscript::ast::{BinOp, Constant, Expr, Ident, Literal, Seq, SourceAst, SwitchCase, UnOp};
use redscript::bundle::{ConstantPool, PoolIndex};
use redscript::definition::{AnyDefinition, Definition}; //, Function, Type};
use redscript::error::Error;

//use crate::Decompiler;

#[derive(Debug)]
struct SortableDef {
    index: PoolIndex<Definition>,
    name: Rc<String>
}

fn get_sorted_definitions<P>(
    pool: &ConstantPool,
    mut predicate: P
) -> Vec<SortableDef> 
where P: FnMut(&Definition) -> bool
{

    let mut defs : Vec<SortableDef> = pool.roots()
        .filter(|(_, def)| predicate(def))
        .map(|(index, def)| SortableDef { index, name: pool.names.get(def.name).unwrap() } )
        .collect::<Vec<SortableDef>>();

    defs.sort_by(|a,b| a.name.cmp(&b.name) );

    return defs;
}

pub fn write_documentation(
    out_path: &Path,
    pool: &ConstantPool
) -> Result<(), Error> {
    let mut output = BufWriter::new(File::create(out_path)?);

    let enums = get_sorted_definitions(pool, |def| matches!(def.value, AnyDefinition::Enum(_)));

    for e in enums {
        writeln!(output, "{}", e.name)?;
    }

    Ok(())
}

pub fn document_definition<W: Write>(
    out: &mut W,
    definition: &Definition,
    pool: &ConstantPool
) -> Result<(), Error> {


    match &definition.value {
        AnyDefinition::Class(class) => {
            writeln!(out)?;
            
            write!(out, "*{} ", class.visibility)?;
            if class.flags.is_abstract() {
                write!(out, "abstract ")?;
            }
            if class.flags.is_final() {
                write!(out, "final ")?;
            }
            if class.flags.is_native() {
                write!(out, "native ")?;
            }
            if class.flags.is_struct() {
                write!(out, "struct")?;
            } else {
                write!(out, "class")?;
            }

            writeln!(out, "*")?;
            write!(out, "# {} ", pool.names.get(definition.name)?)?;
            if !class.base.is_undefined() {
                write!(out, "extends {}", pool.definition_name(class.base)?)?;
            }
            writeln!(out)?;
        },
        _ => {}
    }

    Ok(())
}
