import TextInput from '@Components/input/TextInput';
import NumberInput from '@Components/input/NumberInput';
import { useFormContext } from 'react-hook-form';
import { TestFormInput } from './TestFormContext';

const TestFormFields = () => {
  const { watch } = useFormContext<TestFormInput>();
  const formValues = watch();
  return (
    <>
      <div>{`name: ${formValues.name}`}</div>
      <div>{`price: ${formValues.price}`}</div>
      <TextInput
        name="name"
        label="name"
        registerOptions={{ required: 'ต้องใส่นะ' }}
      />
      <NumberInput
        name="price"
        label="price"
        registerOptions={{
          validate: (value) => value === '1' || 'should be 1',
        }}
      />
    </>
  );
};

export default TestFormFields;
