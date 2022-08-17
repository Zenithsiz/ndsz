//! Directory visitor

// Imports
use zutil::AsciiStrArr;

/// A directory visitor
pub trait Visitor {
	/// Error type
	type Error;

	/// Sub directory visitor type
	type SubDirVisitor<'visitor, 'entry>: Visitor<Error = Self::Error>
	where
		Self: 'visitor;

	/// Visits a file
	fn visit_file(&mut self, name: &AsciiStrArr<0x80>, id: u16) -> Result<(), Self::Error>;

	/// Visits a directory and returns the visitor for the directory
	fn visit_dir<'visitor, 'entry>(
		&'visitor mut self,
		name: &'entry AsciiStrArr<0x80>,
		id: u16,
	) -> Result<Self::SubDirVisitor<'visitor, 'entry>, Self::Error>;
}


impl<V: Visitor> Visitor for &mut V {
	type Error = V::Error;
	type SubDirVisitor<'visitor, 'entry> = V::SubDirVisitor<'visitor, 'entry>
	where
		Self: 'visitor;

	fn visit_file(&mut self, name: &AsciiStrArr<0x80>, id: u16) -> Result<(), Self::Error> {
		(**self).visit_file(name, id)
	}

	fn visit_dir<'visitor, 'entry>(
		&'visitor mut self,
		name: &'entry AsciiStrArr<0x80>,
		id: u16,
	) -> Result<Self::SubDirVisitor<'visitor, 'entry>, Self::Error> {
		(**self).visit_dir(name, id)
	}
}
