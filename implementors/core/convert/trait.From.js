(function() {var implementors = {};
implementors["backtrace"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"struct\" href=\"backtrace/struct.BacktraceFrame.html\" title=\"struct backtrace::BacktraceFrame\">BacktraceFrame</a>&gt;&gt; for <a class=\"struct\" href=\"backtrace/struct.Backtrace.html\" title=\"struct backtrace::Backtrace\">Backtrace</a>","synthetic":false,"types":["backtrace::capture::Backtrace"]}];
implementors["crossbeam_channel"] = [{"text":"impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"crossbeam_channel/struct.SendError.html\" title=\"struct crossbeam_channel::SendError\">SendError</a>&lt;T&gt;&gt; for <a class=\"enum\" href=\"crossbeam_channel/enum.TrySendError.html\" title=\"enum crossbeam_channel::TrySendError\">TrySendError</a>&lt;T&gt;","synthetic":false,"types":["crossbeam_channel::err::TrySendError"]},{"text":"impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"crossbeam_channel/struct.SendError.html\" title=\"struct crossbeam_channel::SendError\">SendError</a>&lt;T&gt;&gt; for <a class=\"enum\" href=\"crossbeam_channel/enum.SendTimeoutError.html\" title=\"enum crossbeam_channel::SendTimeoutError\">SendTimeoutError</a>&lt;T&gt;","synthetic":false,"types":["crossbeam_channel::err::SendTimeoutError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"crossbeam_channel/struct.RecvError.html\" title=\"struct crossbeam_channel::RecvError\">RecvError</a>&gt; for <a class=\"enum\" href=\"crossbeam_channel/enum.TryRecvError.html\" title=\"enum crossbeam_channel::TryRecvError\">TryRecvError</a>","synthetic":false,"types":["crossbeam_channel::err::TryRecvError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"crossbeam_channel/struct.RecvError.html\" title=\"struct crossbeam_channel::RecvError\">RecvError</a>&gt; for <a class=\"enum\" href=\"crossbeam_channel/enum.RecvTimeoutError.html\" title=\"enum crossbeam_channel::RecvTimeoutError\">RecvTimeoutError</a>","synthetic":false,"types":["crossbeam_channel::err::RecvTimeoutError"]}];
implementors["crossbeam_epoch"] = [{"text":"impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"crossbeam_epoch/struct.Owned.html\" title=\"struct crossbeam_epoch::Owned\">Owned</a>&lt;T&gt;&gt; for <a class=\"struct\" href=\"crossbeam_epoch/struct.Atomic.html\" title=\"struct crossbeam_epoch::Atomic\">Atomic</a>&lt;T&gt;","synthetic":false,"types":["crossbeam_epoch::atomic::Atomic"]},{"text":"impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html\" title=\"struct alloc::boxed::Box\">Box</a>&lt;T&gt;&gt; for <a class=\"struct\" href=\"crossbeam_epoch/struct.Atomic.html\" title=\"struct crossbeam_epoch::Atomic\">Atomic</a>&lt;T&gt;","synthetic":false,"types":["crossbeam_epoch::atomic::Atomic"]},{"text":"impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;T&gt; for <a class=\"struct\" href=\"crossbeam_epoch/struct.Atomic.html\" title=\"struct crossbeam_epoch::Atomic\">Atomic</a>&lt;T&gt;","synthetic":false,"types":["crossbeam_epoch::atomic::Atomic"]},{"text":"impl&lt;'g, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"crossbeam_epoch/struct.Shared.html\" title=\"struct crossbeam_epoch::Shared\">Shared</a>&lt;'g, T&gt;&gt; for <a class=\"struct\" href=\"crossbeam_epoch/struct.Atomic.html\" title=\"struct crossbeam_epoch::Atomic\">Atomic</a>&lt;T&gt;","synthetic":false,"types":["crossbeam_epoch::atomic::Atomic"]},{"text":"impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.pointer.html\">*const T</a>&gt; for <a class=\"struct\" href=\"crossbeam_epoch/struct.Atomic.html\" title=\"struct crossbeam_epoch::Atomic\">Atomic</a>&lt;T&gt;","synthetic":false,"types":["crossbeam_epoch::atomic::Atomic"]},{"text":"impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;T&gt; for <a class=\"struct\" href=\"crossbeam_epoch/struct.Owned.html\" title=\"struct crossbeam_epoch::Owned\">Owned</a>&lt;T&gt;","synthetic":false,"types":["crossbeam_epoch::atomic::Owned"]},{"text":"impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html\" title=\"struct alloc::boxed::Box\">Box</a>&lt;T&gt;&gt; for <a class=\"struct\" href=\"crossbeam_epoch/struct.Owned.html\" title=\"struct crossbeam_epoch::Owned\">Owned</a>&lt;T&gt;","synthetic":false,"types":["crossbeam_epoch::atomic::Owned"]},{"text":"impl&lt;'g, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.pointer.html\">*const T</a>&gt; for <a class=\"struct\" href=\"crossbeam_epoch/struct.Shared.html\" title=\"struct crossbeam_epoch::Shared\">Shared</a>&lt;'g, T&gt;","synthetic":false,"types":["crossbeam_epoch::atomic::Shared"]}];
implementors["crossbeam_utils"] = [{"text":"impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;T&gt; for <a class=\"struct\" href=\"crossbeam_utils/struct.CachePadded.html\" title=\"struct crossbeam_utils::CachePadded\">CachePadded</a>&lt;T&gt;","synthetic":false,"types":["crossbeam_utils::cache_padded::CachePadded"]},{"text":"impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;T&gt; for <a class=\"struct\" href=\"crossbeam_utils/sync/struct.ShardedLock.html\" title=\"struct crossbeam_utils::sync::ShardedLock\">ShardedLock</a>&lt;T&gt;","synthetic":false,"types":["crossbeam_utils::sync::sharded_lock::ShardedLock"]}];
implementors["either"] = [{"text":"impl&lt;L, R&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;R, L&gt;&gt; for <a class=\"enum\" href=\"either/enum.Either.html\" title=\"enum either::Either\">Either</a>&lt;L, R&gt;","synthetic":false,"types":["either::Either"]}];
implementors["error_chain"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"error_chain/example_generated/inner/enum.ErrorKind.html\" title=\"enum error_chain::example_generated::inner::ErrorKind\">ErrorKind</a>&gt; for <a class=\"struct\" href=\"error_chain/example_generated/inner/struct.Error.html\" title=\"struct error_chain::example_generated::inner::Error\">Error</a>","synthetic":false,"types":["error_chain::example_generated::inner::Error"]},{"text":"impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;&amp;'a <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.str.html\">str</a>&gt; for <a class=\"struct\" href=\"error_chain/example_generated/inner/struct.Error.html\" title=\"struct error_chain::example_generated::inner::Error\">Error</a>","synthetic":false,"types":["error_chain::example_generated::inner::Error"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>&gt; for <a class=\"struct\" href=\"error_chain/example_generated/inner/struct.Error.html\" title=\"struct error_chain::example_generated::inner::Error\">Error</a>","synthetic":false,"types":["error_chain::example_generated::inner::Error"]},{"text":"impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;&amp;'a <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.str.html\">str</a>&gt; for <a class=\"enum\" href=\"error_chain/example_generated/inner/enum.ErrorKind.html\" title=\"enum error_chain::example_generated::inner::ErrorKind\">ErrorKind</a>","synthetic":false,"types":["error_chain::example_generated::inner::ErrorKind"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>&gt; for <a class=\"enum\" href=\"error_chain/example_generated/inner/enum.ErrorKind.html\" title=\"enum error_chain::example_generated::inner::ErrorKind\">ErrorKind</a>","synthetic":false,"types":["error_chain::example_generated::inner::ErrorKind"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"error_chain/example_generated/inner/struct.Error.html\" title=\"struct error_chain::example_generated::inner::Error\">Error</a>&gt; for <a class=\"enum\" href=\"error_chain/example_generated/inner/enum.ErrorKind.html\" title=\"enum error_chain::example_generated::inner::ErrorKind\">ErrorKind</a>","synthetic":false,"types":["error_chain::example_generated::inner::ErrorKind"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"error_chain/example_generated/inner/struct.Error.html\" title=\"struct error_chain::example_generated::inner::Error\">Error</a>&gt; for <a class=\"struct\" href=\"error_chain/example_generated/struct.Error.html\" title=\"struct error_chain::example_generated::Error\">Error</a>","synthetic":false,"types":["error_chain::example_generated::Error"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html\" title=\"struct std::io::error::Error\">Error</a>&gt; for <a class=\"struct\" href=\"error_chain/example_generated/struct.Error.html\" title=\"struct error_chain::example_generated::Error\">Error</a>","synthetic":false,"types":["error_chain::example_generated::Error"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"error_chain/example_generated/enum.ErrorKind.html\" title=\"enum error_chain::example_generated::ErrorKind\">ErrorKind</a>&gt; for <a class=\"struct\" href=\"error_chain/example_generated/struct.Error.html\" title=\"struct error_chain::example_generated::Error\">Error</a>","synthetic":false,"types":["error_chain::example_generated::Error"]},{"text":"impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;&amp;'a <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.str.html\">str</a>&gt; for <a class=\"struct\" href=\"error_chain/example_generated/struct.Error.html\" title=\"struct error_chain::example_generated::Error\">Error</a>","synthetic":false,"types":["error_chain::example_generated::Error"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>&gt; for <a class=\"struct\" href=\"error_chain/example_generated/struct.Error.html\" title=\"struct error_chain::example_generated::Error\">Error</a>","synthetic":false,"types":["error_chain::example_generated::Error"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"error_chain/example_generated/inner/enum.ErrorKind.html\" title=\"enum error_chain::example_generated::inner::ErrorKind\">ErrorKind</a>&gt; for <a class=\"enum\" href=\"error_chain/example_generated/enum.ErrorKind.html\" title=\"enum error_chain::example_generated::ErrorKind\">ErrorKind</a>","synthetic":false,"types":["error_chain::example_generated::ErrorKind"]},{"text":"impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;&amp;'a <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.str.html\">str</a>&gt; for <a class=\"enum\" href=\"error_chain/example_generated/enum.ErrorKind.html\" title=\"enum error_chain::example_generated::ErrorKind\">ErrorKind</a>","synthetic":false,"types":["error_chain::example_generated::ErrorKind"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>&gt; for <a class=\"enum\" href=\"error_chain/example_generated/enum.ErrorKind.html\" title=\"enum error_chain::example_generated::ErrorKind\">ErrorKind</a>","synthetic":false,"types":["error_chain::example_generated::ErrorKind"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"error_chain/example_generated/struct.Error.html\" title=\"struct error_chain::example_generated::Error\">Error</a>&gt; for <a class=\"enum\" href=\"error_chain/example_generated/enum.ErrorKind.html\" title=\"enum error_chain::example_generated::ErrorKind\">ErrorKind</a>","synthetic":false,"types":["error_chain::example_generated::ErrorKind"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()