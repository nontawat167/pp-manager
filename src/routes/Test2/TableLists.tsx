import { useState } from 'react';
import { createStyles, ScrollArea, Table } from '@mantine/core';
import { dateParse } from '@Utils/date';
import { Sku } from '@Service/types/sku';
import { useAppConfig } from '../../config/ConfigContext';

interface Prop {
  elements: Sku[] | null;
}

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

const TableLists = ({ elements }: Prop) => {
  const { dateFormat } = useAppConfig();
  const { classes, cx } = useStyles();
  const [scrolled, setScrolled] = useState(false);
  const rows =
    elements === null
      ? []
      : elements.map((element) => (
          <tr key={element.id}>
            <td>{element.name}</td>
            <td>{Number(element.price)}</td>
            <td>{element.product_type}</td>
            <td>{dateParse(element.created_at, dateFormat)}</td>
            <td>{dateParse(element.updated_at, dateFormat)}</td>
          </tr>
        ));
  return (
    <div style={{ width: '100%' }}>
      <ScrollArea
        h="70vh"
        onScrollPositionChange={({ y }) => setScrolled(y !== 0)}
      >
        <Table horizontalSpacing="xl" verticalSpacing="sm">
          <thead
            className={cx(classes.header, { [classes.scrolled]: scrolled })}
          >
            <tr>
              <th>ชื่อสินค้า</th>
              <th>ราคาสินค้า</th>
              <th>ประเภทสินค้า</th>
              <th>วันที่สร้าง</th>
              <th>วันที่แก้ไข</th>
            </tr>
          </thead>
          <tbody>{rows}</tbody>
        </Table>
      </ScrollArea>
    </div>
  );
};

export default TableLists;
