import { Table } from '@mantine/core';
import { dateParse } from '@Utils/date';
import { Sku } from '@Service/types/sku';
import { useAppConfig } from '../../config/ConfigContext';

interface Prop {
  elements: Sku[] | null;
}

// interface SkuData {
//   id: string;
//   createdAt: string;
//   updateAt: string;
//   name: string;
//   price: number;
//   type: string;
// }

const TableLists = ({ elements }: Prop) => {
  const { dateFormat } = useAppConfig();
  const rows =
    elements === null
      ? []
      : elements.map((element) => (
          <tr key={element.id}>
            <td>{element.name}</td>
            <td>{Number(element.price)}</td>
            <td>{element.type}</td>
            <td>{dateParse(element.createdAt, dateFormat)}</td>
            <td>{dateParse(element.updatedAt, dateFormat)}</td>
          </tr>
        ));

  return (
    <div style={{ width: '100%' }}>
      <Table horizontalSpacing="xl" verticalSpacing="sm">
        <thead>
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
    </div>
  );
};

export default TableLists;
