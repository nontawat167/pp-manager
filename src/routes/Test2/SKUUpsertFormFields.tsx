import TextInput from '@Components/input/TextInput';
import NumberInput from '@Components/input/NumberInput';

const SKUUpertFormFields = () => {
  return (
    <>
      <TextInput
        name="name"
        label="Name"
        registerOptions={{ required: 'ต้องใส่ชื่อสินค้า' }}
      />
      <NumberInput
        name="price"
        label="Price"
        registerOptions={{ required: 'ต้องระบุจำนวนสินค้า' }}
      />
      <TextInput
        name="type"
        label="Type"
        registerOptions={{ required: 'ต้องใส่ประเภทสินค้า' }}
      />
    </>
  );
};

export default SKUUpertFormFields;
