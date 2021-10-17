use std::fmt;

use super::Vector;

impl<T> fmt::Debug for Vector<T>
where T: fmt::Debug
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_list().entries(self.iter()).finish()
	}
}
