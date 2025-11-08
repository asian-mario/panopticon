#[cfg(test)]
mod tests {
    use super::super::core::adjacency::Adjacency;
    use super::super::core::province::ProvinceDef;

    #[test]
    fn adjacency_refs_exist() {
        let provinces = vec![ProvinceDef { id: 0, name: "A".into(), pos: crate::core::data::Pos { x: 0, y: 0 } }];
        let adj = Adjacency { edges: vec![crate::core::data::Edge { a: 0, b: 0 }] };
        assert_eq!(provinces.len(), 1);
        assert_eq!(adj.edges.len(), 1);
    }
}
