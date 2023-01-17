import {
  createColumnHelper,
  getCoreRowModel,
  useReactTable,
  flexRender,
} from '@tanstack/react-table';
import { useEffect, useMemo, useState } from 'react';
import { getAllProblems, ProblemLeetcode } from '../api/api';
import { PieChart, Pie, ResponsiveContainer, Tooltip } from 'recharts';

export const Table = () => {
  const [problems, setProblems] = useState<ProblemLeetcode[]>([
    { id: 0, problem_name: 'default', url: 'url', has_rust: false },
  ]);

  const columnHelper = createColumnHelper<ProblemLeetcode>();
  const [data01, setdata01] = useState([
    { name: 'Has Rust', value: 300, fill: '#00C49F' },
    { name: 'Does not', value: 400 },
  ]);

  const columns = [
    columnHelper.accessor('id', {
      cell: (info) => info.getValue(),
      footer: (info) => info.column.id,
    }),
    columnHelper.accessor((row) => row.problem_name, {
      id: 'problem_name',
      cell: (info) => <i>{info.getValue()}</i>,
      header: () => <span>Problem Name</span>,
      footer: (info) => info.column.id,
    }),
    columnHelper.accessor('url', {
      header: () => 'Url',
      cell: (info) => info.renderValue(),
      footer: (info) => info.column.id,
    }),

    columnHelper.accessor('has_rust', {
      header: 'Status',
      footer: (info) => info.column.id,
    }),
  ];

  const table = useReactTable({
    data: problems,
    columns: columns,
    getCoreRowModel: getCoreRowModel(),
  });
  useEffect(() => {
    const getProblems = async () => {
      console.log('hola');
      const res = await getAllProblems();
      console.log(res[0]?.url);
      setProblems(res);
      const rust = res.reduce((acc, curr) => {
        if (curr.has_rust) {
          return acc + 1;
        } else {
          return acc;
        }
      }, 0);

      const newData = [
        { name: 'Has Rust', value: rust, fill: '#00C49F' },
        { name: 'Does not', value: res.length - rust },
      ];

      setdata01(newData);
    };
    getProblems();
  }, []);
  return (
    <div>
      <PieChart width={500} height={500}>
        <Pie data={data01} dataKey='value' />
        <Tooltip />
      </PieChart>
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
    </div>
  );
};
