/*! # Read environment variables by lower, upper case or case-insensitive keys
 *
 * This crate provides an interface similar to `std::env` but whose keys are
 * converted to lower or UPPER case, or presented in a way that can be compared
 * regardless of case, either ASCII or Unicode (with feature `unicode` enabled,
 * using [unicase](https://docs.rs/unicase)).
 *
 * Only variants of [`env::var`] and [`env::vars`] are provided, since `OsStr`s
 * are not necessarily human-readable character strings. 
 */

use std::env;
#[cfg(feature = "unicode")]
use unicase::UniCase;

/** Helper for uncased comparison. */
pub struct UncasedPartialEq(
	#[cfg(feature = "unicode")]
	UniCase<String>,
	#[cfg(not(feature = "unicode"))]
	String,
);

#[cfg(feature = "unicode")]
impl<S: AsRef<str>> PartialEq<S> for UncasedPartialEq {
	fn eq(&self, other: &S) -> bool {
		self.0 == other.into()
	}
}

#[cfg(not(feature = "unicode"))]
impl<S: AsRef<str>> PartialEq<S> for UncasedPartialEq {
	fn eq(&self, other: &S) -> bool {
		self.0.eq_ignore_ascii_case(other.as_ref())
	}
}

#[cfg(not(feature = "unicode"))]
impl std::ops::Deref for UncasedPartialEq {
	type Target = String;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

/** Iterator of environment variables whose keys can be compared regardless
 * of case.
 */
#[derive(Debug)]
pub struct UncasedVars(env::Vars);

#[cfg(feature = "unicode")]
impl Iterator for UncasedVars {
	type Item = (UncasedPartialEq, String);

	fn next(&mut self) -> Option<Self::Item> {
		self.0.next().map(|(k, v)| (UncasedPartialEq(UniCase::new(k)), v))
	}
}

#[cfg(not(feature = "unicode"))]
impl Iterator for UncasedVars {
	type Item = (UncasedPartialEq, String);

	fn next(&mut self) -> Option<Self::Item> {
		self.0.next().map(|(k, v)| (UncasedPartialEq(k), v))
	}
}

/** Get an [`UncasedVars`] whose keys can be compared regardless of case.
 */
pub fn uncased_vars() -> UncasedVars {
	UncasedVars(env::vars())
}

/** Get value by a key like [`env::var`], but accept a key of any case of
 * `AsRef<str>` rather than `AsRef<OsStr>`.
 */
pub fn uncased_var<K: AsRef<str>>(key: K) -> Result<String, env::VarError> {
	let key = key.as_ref();
	uncased_vars().find(|(k, _)| k == &key).map(|(_, v)| v).ok_or(env::VarError::NotPresent)
}

/** Iterator of environment variables whose keys are lowercased.
 */
#[derive(Debug)]
pub struct LowerVars(env::Vars);

impl Iterator for LowerVars {
	type Item = (String, String);

	fn next(&mut self) -> Option<Self::Item> {
		self.0.next().map(|(k, v)| (
			#[cfg(feature = "unicode")]
			k.to_lowercase(),
			#[cfg(not(feature = "unicode"))]
			k.to_ascii_lowercase(),
		v))
	}
}

/** Get an iterator of environment variables like [`env::vars`], but
 * with lowercased keys.
 */
pub fn lower_vars() -> LowerVars {
	LowerVars(env::vars())
}

/** Get value by a key like [`env::var`], but accept a lowercased key of
 * `AsRef<str>` rather than `AsRef<OsStr>`.
 */
pub fn lower_var<K: AsRef<str>>(key: K) -> Result<String, env::VarError> {
	let key = key.as_ref();
	lower_vars().find(|(k, _)| k == key).map(|(_, v)| v).ok_or(env::VarError::NotPresent)
}

/** Iterator of environment variables whose keys are UPPERCASED.
 */
#[derive(Debug)]
pub struct UpperVars(env::Vars);

impl Iterator for UpperVars {
	type Item = (String, String);

	fn next(&mut self) -> Option<Self::Item> {
		self.0.next().map(|(k, v)| (
			#[cfg(feature = "unicode")]
			k.to_uppercase(),
			#[cfg(not(feature = "unicode"))]
			k.to_ascii_uppercase(),
		v))
	}
}

/** Get an iterator of environment variables like [`env::vars`], but
 * with uppercased keys.
 */
pub fn upper_vars() -> UpperVars {
	UpperVars(env::vars())
}

/** Get value by a key like [`env::var`], but accept an UPPERCASED key of
 * `AsRef<str>` rather than `AsRef<OsStr>`.
 */
pub fn upper_var<K: AsRef<str>>(key: K) -> Result<String, env::VarError> {
	let key = key.as_ref();
	upper_vars().find(|(k, _)| k == key).map(|(_, v)| v).ok_or(env::VarError::NotPresent)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn uncased() {
		env::set_var("HELLO", "world");
		assert_eq!(uncased_var("Hello"), Ok("world".to_string()));
	}

	#[test]
	fn low() {
		env::set_var("HELLO", "world");
		assert_eq!(lower_var("hello"), Ok("world".to_string()));
	}

	#[test]
	fn up() {
		env::set_var("hello", "world");
		assert_eq!(upper_var("HELLO"), Ok("world".to_string()));
	}

	#[cfg(feature = "unicode")]
	#[test]
	fn unicode() {
		env::set_var("Maße", "42");
		assert_eq!(uncased_var("mAßE"), Ok("42".to_string()));
		assert_eq!(lower_var("maße"), Ok("42".to_string()));
		assert_eq!(upper_var("MASSE"), Ok("42".to_string()));
	}
}

