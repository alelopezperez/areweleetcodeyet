import { useEffect, useState } from 'react';
import {
  useReactTable,
  createColumnHelper,
  getCoreRowModel,
  buildHeaderGroups,
  flexRender,
  getPaginationRowModel,
} from '@tanstack/react-table';
import { getAllProblems, LeetcodeQuestion } from './api/api';

const columnHelper = createColumnHelper<LeetcodeQuestion>();

const columns = [
  columnHelper.accessor('id', {
    cell: (info) => info.getValue(),
    footer: (info) => info.column.id,
  }),
  columnHelper.accessor('problem_name', {
    header: 'Problem Name',
    cell: (info) => info.getValue(),
    footer: (info) => info.column.id,
  }),
  columnHelper.accessor('url', {
    cell: (info) => info.getValue(),
    footer: (info) => info.column.id,
  }),
  columnHelper.accessor('has_rust', {
    header: 'Status',
    cell: (info) => (info.getValue() ? 'Yes' : 'No'),
    footer: (info) => info.column.id,
  }),
];

const App = () => {
  const [data, setData] = useState<LeetcodeQuestion[]>([
    { id: NaN, problem_name: '', url: '', has_rust: false },
  ]);
  useEffect(() => {
    table.setPageSize(10);
    const initData = async () => {
      const response = await getAllProblems();
      setData(response);
    };
    initData();
  }, []);

  const table = useReactTable({
    data: data,
    columns: columns,
    getCoreRowModel: getCoreRowModel(),
    getPaginationRowModel: getPaginationRowModel(),
  });

  return (
    <div className='flex justify-center flex-col items-center'>
      <h1 className='text-3xl text-green-600'>Are We Leetcode Yet?</h1>
      <p className='text-3xl '>Keeping track of leetcode questions</p>

      <table>
        <thead>
          {table.getHeaderGroups().map((headerGroup) => (
            <tr key={headerGroup.id}>
              {headerGroup.headers.map((header) => (
                <th key={header.id}>
                  {header.isPlaceholder
                    ? null
                    : flexRender(
                        header.column.columnDef.header,
                        header.getContext()
                      )}
                </th>
              ))}
            </tr>
          ))}
        </thead>
        <tbody>
          {table.getRowModel().rows.map((row) => (
            <tr key={row.id}>
              {row.getVisibleCells().map((cell) => (
                <td key={cell.id}>
                  {flexRender(cell.column.columnDef.cell, cell.getContext())}
                </td>
              ))}
            </tr>
          ))}
        </tbody>
      </table>

      <div className='flex '>
        <button
          className='border rounded p-1'
          onClick={() => table.previousPage()}
          disabled={!table.getCanPreviousPage()}
        >
          {'<'}
        </button>
        <button
          className='border rounded p-1'
          onClick={() => table.nextPage()}
          disabled={!table.getCanNextPage()}
        >
          {'>'}
        </button>
        <span className='flex items-center gap-1'>
          <div>Page</div>
          <strong>
            {table.getState().pagination.pageIndex + 1} of{' '}
            {table.getPageCount()}
          </strong>
        </span>
        <select
          onChange={(e) => {
            table.setPageSize(Number(e.target.value));
          }}
        >
          <option>10</option>
          <option>20</option>
        </select>
      </div>
    </div>
  );
};

export default App;
