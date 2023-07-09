import {
  createStyles,
  Paper,
  ScrollArea,
  Table as MantineTable,
} from '@mantine/core';
import { ObjectWithID } from '@Utils/types';
import { useState } from 'react';

interface TableProperty {
  data: ObjectWithID[];
  columns: ColumnProperty[];
}

interface ColumnProperty {
  header: string;
  field: string;
}

const genarateTableHeader = (columns: ColumnProperty[]) => {
  const headers = columns.map((column) => (
    <th key={column.header}>{column.header}</th>
  ));
  return <tr>{headers}</tr>;
};

const genarateTableRow = (datas: ObjectWithID[], columns: ColumnProperty[]) => {
  const rows = datas.map((data) => (
    <tr key={data.id}>
      {columns.map(({ field }) => (
        <td key={`${field}${data.id}`}>{data[field]}</td>
      ))}
    </tr>
  ));
  return rows;
};

const useStyles = createStyles((theme) => ({
  header: {
    position: 'sticky',
    top: 0,
    backgroundColor:
      theme.colorScheme === 'dark' ? theme.colors.dark[7] : theme.white,
    transition: 'box-shadow 150ms ease',
  },

  scrolled: {
    boxShadow: theme.shadows.sm,
  },
}));

const Table = ({ data, columns }: TableProperty) => {
  const tableHeader = genarateTableHeader(columns);
  const tableRow = genarateTableRow(data, columns);
  const { classes, cx } = useStyles();
  const [scrolled, setScrolled] = useState(false);
  return (
    <div style={{ width: '100%' }}>
      <Paper shadow="xs" radius="md" p="md" mt="md">
        <ScrollArea
          h="70vh"
          onScrollPositionChange={({ y }) => setScrolled(y !== 0)}
        >
          <MantineTable horizontalSpacing="xl" verticalSpacing="sm">
            <thead
              className={cx(classes.header, { [classes.scrolled]: scrolled })}
            >
              {tableHeader}
            </thead>
            <tbody>{tableRow}</tbody>
          </MantineTable>
        </ScrollArea>
      </Paper>
    </div>
  );
};

export default Table;
