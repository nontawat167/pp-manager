import { Checkbox, Table } from '@mantine/core';
import { dateParse } from '@Utils/date';
import { useAppConfig } from '../../config/ConfigContext';

interface prop {
  elements: skuData[];
}

interface skuData {
  id: string;
  createdAt: string;
  updateAt: string;
  name: string;
  price: number;
  type: string;
}

const TableLists = ({ elements }: prop) => {
  const { dateFormat } = useAppConfig();
  const rows = elements.map((element) => (
    <tr key={element.id}>
      <td>{element.name}</td>
      <td>{element.price}</td>
      <td>{element.type}</td>
      <td>{dateParse(element.createdAt,dateFormat)}</td>
      <td>{dateParse(element.updateAt,dateFormat)}</td>
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
