/// Dijkstra's Shortest Path First algorithm
trait SPF {
    type LenTable;
    fn spf(&mut self) -> Self::LenTable;
}

impl<'a, T: GraphTraverser<'a>> SPF for DfsIterStateHolder<'a, T::Item>  
where 
    <T as GraphTraverser<'a>>::Item: Eq + Hash,
{
    type LenTable = HashMap<T::Item, usize>;

    fn spf(&mut self) -> Self::LenTable {
        let table = Self::LenTable::new();
        // let item = self.next();
        // let lenght = length_accessor(item);
        // table[item] = lenght;

        return table;
    }
}
