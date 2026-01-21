        let lines_start = range.start.lines_index;
        let lines_end = range.end.lines_index;
        if lines_start <= lines_end && lines_end <= previous_frame.used_lines.len() {
            for key in &previous_frame.used_lines[lines_start..lines_end] {
                if let Some((key, line)) = previous_frame.lines.remove_entry(key) {
                    current_frame.lines.insert(key, line);
                }
                current_frame.used_lines.push(key.clone());
        let wrapped_start = range.start.wrapped_lines_index;
        let wrapped_end = range.end.wrapped_lines_index;
        if wrapped_start <= wrapped_end && wrapped_end <= previous_frame.used_wrapped_lines.len() {
            for key in &previous_frame.used_wrapped_lines[wrapped_start..wrapped_end] {
                if let Some((key, line)) = previous_frame.wrapped_lines.remove_entry(key) {
                    current_frame.wrapped_lines.insert(key, line);
                }
                current_frame.used_wrapped_lines.push(key.clone());

#[cfg(test)]
mod tests {
    use super::{LineLayoutCache, LineLayoutIndex};
    use crate::platform::NoopTextSystem;
    use std::sync::Arc;

    #[test]
    fn reuse_layouts_ignores_out_of_range_indices() {
        let cache = LineLayoutCache::new(Arc::new(NoopTextSystem::new()));
        let range = LineLayoutIndex {
            lines_index: 2,
            wrapped_lines_index: 0,
        }..LineLayoutIndex {
            lines_index: 2,
            wrapped_lines_index: 0,
        };

        cache.reuse_layouts(range);

        let current_frame = cache.current_frame.read();
        assert!(current_frame.used_lines.is_empty());
        assert!(current_frame.used_wrapped_lines.is_empty());
    }
}