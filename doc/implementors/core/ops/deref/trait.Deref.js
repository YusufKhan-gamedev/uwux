(function() {var implementors = {};
implementors["aero_kernel"] = [{"text":"impl Deref for <a class=\"struct\" href=\"aero_kernel/arch/x86_64/interrupts/struct.PIC_CONTROLLER.html\" title=\"struct aero_kernel::arch::x86_64::interrupts::PIC_CONTROLLER\">PIC_CONTROLLER</a>","synthetic":false,"types":["aero_kernel::arch::x86_64::interrupts::PIC_CONTROLLER"]},{"text":"impl Deref for <a class=\"struct\" href=\"aero_kernel/arch/x86_64/interrupts/struct.APIC_CONTROLLER.html\" title=\"struct aero_kernel::arch::x86_64::interrupts::APIC_CONTROLLER\">APIC_CONTROLLER</a>","synthetic":false,"types":["aero_kernel::arch::x86_64::interrupts::APIC_CONTROLLER"]},{"text":"impl Deref for <a class=\"struct\" href=\"aero_kernel/arch/x86_64/interrupts/struct.INTERRUPT_CONTROLLER.html\" title=\"struct aero_kernel::arch::x86_64::interrupts::INTERRUPT_CONTROLLER\">INTERRUPT_CONTROLLER</a>","synthetic":false,"types":["aero_kernel::arch::x86_64::interrupts::INTERRUPT_CONTROLLER"]},{"text":"impl Deref for <a class=\"struct\" href=\"aero_kernel/drivers/mouse/struct.MOUSE.html\" title=\"struct aero_kernel::drivers::mouse::MOUSE\">MOUSE</a>","synthetic":false,"types":["aero_kernel::drivers::mouse::MOUSE"]},{"text":"impl&lt;K:&nbsp;<a class=\"trait\" href=\"aero_kernel/fs/cache/trait.CacheKey.html\" title=\"trait aero_kernel::fs::cache::CacheKey\">CacheKey</a>, V:&nbsp;<a class=\"trait\" href=\"aero_kernel/fs/cache/trait.Cacheable.html\" title=\"trait aero_kernel::fs::cache::Cacheable\">Cacheable</a>&lt;K&gt;&gt; Deref for <a class=\"struct\" href=\"aero_kernel/fs/cache/struct.CacheItem.html\" title=\"struct aero_kernel::fs::cache::CacheItem\">CacheItem</a>&lt;K, V&gt;","synthetic":false,"types":["aero_kernel::fs::cache::CacheItem"]},{"text":"impl Deref for <a class=\"struct\" href=\"aero_kernel/fs/cache/struct.CachedINode.html\" title=\"struct aero_kernel::fs::cache::CachedINode\">CachedINode</a>","synthetic":false,"types":["aero_kernel::fs::cache::CachedINode"]},{"text":"impl Deref for <a class=\"struct\" href=\"aero_kernel/fs/devfs/struct.DEV_FILESYSTEM.html\" title=\"struct aero_kernel::fs::devfs::DEV_FILESYSTEM\">DEV_FILESYSTEM</a>","synthetic":false,"types":["aero_kernel::fs::devfs::DEV_FILESYSTEM"]},{"text":"impl Deref for <a class=\"struct\" href=\"aero_kernel/fs/struct.MOUNT_MANAGER.html\" title=\"struct aero_kernel::fs::MOUNT_MANAGER\">MOUNT_MANAGER</a>","synthetic":false,"types":["aero_kernel::fs::MOUNT_MANAGER"]}];
implementors["intrusive_collections"] = [{"text":"impl&lt;T:&nbsp;?Sized&gt; Deref for <a class=\"struct\" href=\"intrusive_collections/struct.UnsafeRef.html\" title=\"struct intrusive_collections::UnsafeRef\">UnsafeRef</a>&lt;T&gt;","synthetic":false,"types":["intrusive_collections::unsafe_ref::UnsafeRef"]}];
implementors["linked_list_allocator"] = [{"text":"impl Deref for <a class=\"struct\" href=\"linked_list_allocator/struct.LockedHeap.html\" title=\"struct linked_list_allocator::LockedHeap\">LockedHeap</a>","synthetic":false,"types":["linked_list_allocator::LockedHeap"]}];
implementors["lock_api"] = [{"text":"impl&lt;'a, R:&nbsp;<a class=\"trait\" href=\"lock_api/trait.RawMutex.html\" title=\"trait lock_api::RawMutex\">RawMutex</a> + 'a, T:&nbsp;?Sized + 'a&gt; Deref for <a class=\"struct\" href=\"lock_api/struct.MutexGuard.html\" title=\"struct lock_api::MutexGuard\">MutexGuard</a>&lt;'a, R, T&gt;","synthetic":false,"types":["lock_api::mutex::MutexGuard"]},{"text":"impl&lt;'a, R:&nbsp;<a class=\"trait\" href=\"lock_api/trait.RawMutex.html\" title=\"trait lock_api::RawMutex\">RawMutex</a> + 'a, T:&nbsp;?Sized + 'a&gt; Deref for <a class=\"struct\" href=\"lock_api/struct.MappedMutexGuard.html\" title=\"struct lock_api::MappedMutexGuard\">MappedMutexGuard</a>&lt;'a, R, T&gt;","synthetic":false,"types":["lock_api::mutex::MappedMutexGuard"]},{"text":"impl&lt;'a, R:&nbsp;<a class=\"trait\" href=\"lock_api/trait.RawMutex.html\" title=\"trait lock_api::RawMutex\">RawMutex</a> + 'a, G:&nbsp;<a class=\"trait\" href=\"lock_api/trait.GetThreadId.html\" title=\"trait lock_api::GetThreadId\">GetThreadId</a> + 'a, T:&nbsp;?Sized + 'a&gt; Deref for <a class=\"struct\" href=\"lock_api/struct.ReentrantMutexGuard.html\" title=\"struct lock_api::ReentrantMutexGuard\">ReentrantMutexGuard</a>&lt;'a, R, G, T&gt;","synthetic":false,"types":["lock_api::remutex::ReentrantMutexGuard"]},{"text":"impl&lt;'a, R:&nbsp;<a class=\"trait\" href=\"lock_api/trait.RawMutex.html\" title=\"trait lock_api::RawMutex\">RawMutex</a> + 'a, G:&nbsp;<a class=\"trait\" href=\"lock_api/trait.GetThreadId.html\" title=\"trait lock_api::GetThreadId\">GetThreadId</a> + 'a, T:&nbsp;?Sized + 'a&gt; Deref for <a class=\"struct\" href=\"lock_api/struct.MappedReentrantMutexGuard.html\" title=\"struct lock_api::MappedReentrantMutexGuard\">MappedReentrantMutexGuard</a>&lt;'a, R, G, T&gt;","synthetic":false,"types":["lock_api::remutex::MappedReentrantMutexGuard"]},{"text":"impl&lt;'a, R:&nbsp;<a class=\"trait\" href=\"lock_api/trait.RawRwLock.html\" title=\"trait lock_api::RawRwLock\">RawRwLock</a> + 'a, T:&nbsp;?Sized + 'a&gt; Deref for <a class=\"struct\" href=\"lock_api/struct.RwLockReadGuard.html\" title=\"struct lock_api::RwLockReadGuard\">RwLockReadGuard</a>&lt;'a, R, T&gt;","synthetic":false,"types":["lock_api::rwlock::RwLockReadGuard"]},{"text":"impl&lt;'a, R:&nbsp;<a class=\"trait\" href=\"lock_api/trait.RawRwLock.html\" title=\"trait lock_api::RawRwLock\">RawRwLock</a> + 'a, T:&nbsp;?Sized + 'a&gt; Deref for <a class=\"struct\" href=\"lock_api/struct.RwLockWriteGuard.html\" title=\"struct lock_api::RwLockWriteGuard\">RwLockWriteGuard</a>&lt;'a, R, T&gt;","synthetic":false,"types":["lock_api::rwlock::RwLockWriteGuard"]},{"text":"impl&lt;'a, R:&nbsp;<a class=\"trait\" href=\"lock_api/trait.RawRwLockUpgrade.html\" title=\"trait lock_api::RawRwLockUpgrade\">RawRwLockUpgrade</a> + 'a, T:&nbsp;?Sized + 'a&gt; Deref for <a class=\"struct\" href=\"lock_api/struct.RwLockUpgradableReadGuard.html\" title=\"struct lock_api::RwLockUpgradableReadGuard\">RwLockUpgradableReadGuard</a>&lt;'a, R, T&gt;","synthetic":false,"types":["lock_api::rwlock::RwLockUpgradableReadGuard"]},{"text":"impl&lt;'a, R:&nbsp;<a class=\"trait\" href=\"lock_api/trait.RawRwLock.html\" title=\"trait lock_api::RawRwLock\">RawRwLock</a> + 'a, T:&nbsp;?Sized + 'a&gt; Deref for <a class=\"struct\" href=\"lock_api/struct.MappedRwLockReadGuard.html\" title=\"struct lock_api::MappedRwLockReadGuard\">MappedRwLockReadGuard</a>&lt;'a, R, T&gt;","synthetic":false,"types":["lock_api::rwlock::MappedRwLockReadGuard"]},{"text":"impl&lt;'a, R:&nbsp;<a class=\"trait\" href=\"lock_api/trait.RawRwLock.html\" title=\"trait lock_api::RawRwLock\">RawRwLock</a> + 'a, T:&nbsp;?Sized + 'a&gt; Deref for <a class=\"struct\" href=\"lock_api/struct.MappedRwLockWriteGuard.html\" title=\"struct lock_api::MappedRwLockWriteGuard\">MappedRwLockWriteGuard</a>&lt;'a, R, T&gt;","synthetic":false,"types":["lock_api::rwlock::MappedRwLockWriteGuard"]}];
implementors["scopeguard"] = [{"text":"impl&lt;T, F, S&gt; Deref for <a class=\"struct\" href=\"scopeguard/struct.ScopeGuard.html\" title=\"struct scopeguard::ScopeGuard\">ScopeGuard</a>&lt;T, F, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;F: FnOnce(T),<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"scopeguard/trait.Strategy.html\" title=\"trait scopeguard::Strategy\">Strategy</a>,&nbsp;</span>","synthetic":false,"types":["scopeguard::ScopeGuard"]}];
implementors["spin"] = [{"text":"impl&lt;T, F:&nbsp;FnOnce() -&gt; T, R:&nbsp;<a class=\"trait\" href=\"spin/relax/trait.RelaxStrategy.html\" title=\"trait spin::relax::RelaxStrategy\">RelaxStrategy</a>&gt; Deref for <a class=\"struct\" href=\"spin/lazy/struct.Lazy.html\" title=\"struct spin::lazy::Lazy\">Lazy</a>&lt;T, F, R&gt;","synthetic":false,"types":["spin::lazy::Lazy"]},{"text":"impl&lt;'a, T:&nbsp;?Sized&gt; Deref for <a class=\"struct\" href=\"spin/mutex/spin/struct.SpinMutexGuard.html\" title=\"struct spin::mutex::spin::SpinMutexGuard\">SpinMutexGuard</a>&lt;'a, T&gt;","synthetic":false,"types":["spin::mutex::spin::SpinMutexGuard"]},{"text":"impl&lt;'a, T:&nbsp;?Sized&gt; Deref for <a class=\"struct\" href=\"spin/mutex/struct.MutexGuard.html\" title=\"struct spin::mutex::MutexGuard\">MutexGuard</a>&lt;'a, T&gt;","synthetic":false,"types":["spin::mutex::MutexGuard"]},{"text":"impl&lt;'rwlock, T:&nbsp;?Sized&gt; Deref for <a class=\"struct\" href=\"spin/rwlock/struct.RwLockReadGuard.html\" title=\"struct spin::rwlock::RwLockReadGuard\">RwLockReadGuard</a>&lt;'rwlock, T&gt;","synthetic":false,"types":["spin::rwlock::RwLockReadGuard"]},{"text":"impl&lt;'rwlock, T:&nbsp;?Sized, R&gt; Deref for <a class=\"struct\" href=\"spin/rwlock/struct.RwLockUpgradableGuard.html\" title=\"struct spin::rwlock::RwLockUpgradableGuard\">RwLockUpgradableGuard</a>&lt;'rwlock, T, R&gt;","synthetic":false,"types":["spin::rwlock::RwLockUpgradableGuard"]},{"text":"impl&lt;'rwlock, T:&nbsp;?Sized, R&gt; Deref for <a class=\"struct\" href=\"spin/rwlock/struct.RwLockWriteGuard.html\" title=\"struct spin::rwlock::RwLockWriteGuard\">RwLockWriteGuard</a>&lt;'rwlock, T, R&gt;","synthetic":false,"types":["spin::rwlock::RwLockWriteGuard"]}];
implementors["x86_64"] = [{"text":"impl Deref for <a class=\"struct\" href=\"x86_64/structures/idt/struct.InterruptStackFrame.html\" title=\"struct x86_64::structures::idt::InterruptStackFrame\">InterruptStackFrame</a>","synthetic":false,"types":["x86_64::structures::idt::InterruptStackFrame"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()